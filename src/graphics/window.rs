extern crate glium;

use std::rc::Rc;
use std::cell::RefCell;

/// Window structure
///
/// Stores the raw handle to the windowing system and the graphics rendering
/// context
pub struct Window {
    facade: glium::backend::glutin_backend::GlutinFacade
}

/// WindowBuilder
///
/// Builds a Window
pub struct WindowBuilder {
    width: u32,
    height: u32,
    title: String,
    vsync: bool,
    depth: u8,
    stencil: u8,
    srgb: bool
}
impl WindowBuilder {
    /// Constructs a new WindowBuilder
    pub fn new() -> Self {
        WindowBuilder {
            width: 1024,
            height: 768,
            title: String::from("Oxygine Game Engine Window"),
            vsync: true,
            depth: 24,
            stencil: 8,
            srgb: true
        }
    }
    /// Requests a size other than default (1024x768)
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    /// Requests a title other than default ("Oxygine Game Engine Window")
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = String::from(title);
        self
    }
    /// Requests vsync to be on or off (default: on)
    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }
    /// Requests a depth buffer bit size other than default (24 bits)
    pub fn with_depth(mut self, depth_bits: u8) -> Self {
        self.depth = depth_bits;
        self
    }
    /// Requests a stencil buffer bit size other than default (8 bits)
    pub fn with_stencil(mut self, stencil_bits: u8) -> Self {
        self.stencil = stencil_bits;
        self
    }
    /// Requests to use sRGB color space (default: yes)
    pub fn with_srgb(mut self, srgb: bool) -> Self {
        self.srgb = srgb;
        self
    }

    /// Builds the Window and returns it as an Rc<RefCell<Window>>
    ///
    /// The Window isn't returned directly to allow for many systems to be able
    /// to utilize the Window simultaneously.
    pub fn build(self) -> Rc<RefCell<Window>> {
        use graphics::window::glium::DisplayBuild;

        let mut builder = glium::glutin::WindowBuilder::new();

        builder = builder.with_dimensions(self.width, self.height)
                         .with_title(self.title)
                         .with_depth_buffer(self.depth)
                         .with_stencil_buffer(self.stencil);

        if self.vsync {
            builder = builder.with_vsync();
        }
        if self.srgb {
            builder = builder.with_srgb(Some(true));
        }

        let facade = builder.build_glium().unwrap();

        Rc::new(RefCell::new(
            Window {
                facade: facade
            }
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn open() {
        #[allow(unused_variables)]
        let wnd = super::WindowBuilder::new().build();
    }
}
