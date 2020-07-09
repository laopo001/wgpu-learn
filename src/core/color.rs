#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        return Color {
            r: r,
            g: g,
            b: b,
            a: a,
        };
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        unsafe {
            return std::mem::transmute(self);
        }
    }
}

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        unsafe {
            return [self.r, self.g, self.a];
        }
    }
}

impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }
}
