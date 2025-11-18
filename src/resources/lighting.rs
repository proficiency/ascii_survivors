use bevy::prelude::*;

/// Shared GPU texture & buffer used to composite multi-colored lighting
/// over the ASCII terminal output.
#[derive(Resource)]
pub struct LightingOverlay {
    pub handle: Handle<Image>,
    pub size: UVec2,
    pub texture_size: UVec2,
    pub pixel_scale: u32,
    pub ambient_color: LinearRgba,
    pub buffer: Vec<LinearRgba>,
}

impl LightingOverlay {
    pub fn buffer_dimensions(&self) -> (usize, usize) {
        (
            (self.texture_size.x) as usize,
            (self.texture_size.y) as usize,
        )
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.ambient_color);
    }
}
