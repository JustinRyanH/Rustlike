pub mod raw;
pub mod shader;
pub mod error;

pub struct GlContext {
    pub bg_color: [f32; 3],
}

impl GlContext {
    pub fn clear(&self) {
        unsafe {
            raw::ClearColor(self.bg_color[0], self.bg_color[1], self.bg_color[2], 1.);
            raw::Clear(raw::COLOR_BUFFER_BIT);
        }
    }
}


impl Default for GlContext {
    fn default() -> GlContext {
        GlContext {
            bg_color: [154./255., 206./255., 235./255.],
        }
    }
}
