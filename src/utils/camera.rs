use macroquad::{is_key_down, mouse_position, screen_height, screen_width, vec2, KeyCode, Vec2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    target: Vec2,
    zoom: Vec2,
}

impl Camera {
    pub const fn get(&self) -> (Vec2, Vec2) {
        (self.target, self.zoom)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let starting_zoom = 0.10;
        Self {
            target: vec2(0.0, 0.0),
            zoom: vec2(
                starting_zoom,
                starting_zoom * screen_width() / screen_height(),
            ),
        }
    }
}

/// Get the mouse coordinates inside the game world.
pub fn relative_mouse_position(camera: &Camera) -> Vec2 {
    // Takes the mouse coordinates on window and translates that
    // to game world coordinates.
    let mouse = mouse_position();
    Vec2::new(
        ((mouse.0 - screen_width() / 2.0) / (screen_width() / 2.0) / camera.zoom.x())
            + camera.target.x(),
        ((-mouse.1 + screen_height() / 2.0)
            / (screen_height() / 2.0)
            / camera.zoom.x()
            / (screen_width() / screen_height()))
            + camera.target.y(),
    )
}

/// Get and handle the input related to the camera.
pub fn scroll(camera: &mut Camera, scroll_speed: f32, zoom_speed: f32) {
    // Move the camera:
    // UP
    if is_key_down(KeyCode::Comma) {
        camera
            .target
            .set_y(camera.target.y() + scroll_speed / camera.zoom.x())
    }
    // DOWN
    if is_key_down(KeyCode::O) {
        camera
            .target
            .set_y(camera.target.y() - scroll_speed / camera.zoom.x())
    }
    // LEFT
    if is_key_down(KeyCode::A) {
        camera
            .target
            .set_x(camera.target.x() - scroll_speed / camera.zoom.x())
    }
    // RIGHT
    if is_key_down(KeyCode::E) {
        camera
            .target
            .set_x(camera.target.x() + scroll_speed / camera.zoom.x())
    }
    // Change the camera zoom:
    // Further
    if is_key_down(KeyCode::Apostrophe) {
        camera.zoom.set_x(camera.zoom.x() * zoom_speed);
        camera.zoom.set_y(camera.zoom.y() * zoom_speed);
    }
    // Closer
    if is_key_down(KeyCode::Period) {
        camera.zoom.set_x(camera.zoom.x() / zoom_speed);
        camera.zoom.set_y(camera.zoom.y() / zoom_speed);
    }
}
