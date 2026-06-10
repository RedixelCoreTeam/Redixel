/// An RGBA colour with `f32` components in the range `[0.0, 1.0]`.
///
/// Used for clear colours, tints, and draw calls.
/// Stored as linear colour — apply gamma correction at the shader level.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Self = Self::rgba(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);
    pub const RED: Self = Self::rgba(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::rgba(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::rgba(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Self = Self::rgba(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Self = Self::rgba(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::rgba(1.0, 0.0, 1.0, 1.0);
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);

    #[inline]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Creates a colour from 8-bit RGBA components (0–255).
    #[inline]
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
    }

    /// Creates a colour from a hex code (`0xRRGGBBAA`).
    #[inline]
    pub fn from_hex(hex: u32) -> Self {
        Self::from_rgba8(
            ((hex >> 24) & 0xFF) as u8,
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
        )
    }

    /// Returns `[r, g, b, a]` — compatible with WGPU colour attachment clear values.
    #[inline]
    pub fn to_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Blends linearly toward `other` by factor `t ∈ [0, 1]`.
    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let lerp = |a: f32, b: f32| -> f32 { a + (b - a) * t };
        Self::rgba(
            lerp(self.r, other.r),
            lerp(self.g, other.g),
            lerp(self.b, other.b),
            lerp(self.a, other.a),
        )
    }

    /// Returns the colour with a different alpha.
    #[inline]
    pub fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

/// Converts to a `wgpu::Color` for use in render pass clear values.
impl From<Color> for wgpu::Color {
    fn from(c: Color) -> Self {
        wgpu::Color {
            r: c.r as f64,
            g: c.g as f64,
            b: c.b as f64,
            a: c.a as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex() {
        let c: Color = Color::from_hex(0xFF8000FF);
        assert!((c.r - 1.0).abs() < 1e-3);
        assert!((c.g - 0.502).abs() < 1e-3);
        assert!((c.b - 0.0).abs() < 1e-3);
        assert!((c.a - 1.0).abs() < 1e-3);
    }

    #[test]
    fn lerp_halfway() {
        let c: Color = Color::BLACK.lerp(Color::WHITE, 0.5);
        assert!((c.r - 0.5).abs() < 1e-6);
    }
}
