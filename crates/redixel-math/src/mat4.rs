/// A 4×4 column-major matrix used for coordinate transforms.
///
/// Stored as `cols[col][row]` — identical to WGSL's `mat4x4<f32>` memory layout,
/// so it can be copied directly into a uniform buffer with `bytemuck`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub cols: [[f32; 4]; 4],
}

impl Mat4 {
    pub const IDENTITY: Self = Self {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    /// Constructs from four column vectors (each a `[f32; 4]`).
    pub fn from_cols(c0: [f32; 4], c1: [f32; 4], c2: [f32; 4], c3: [f32; 4]) -> Self {
        Self { cols: [c0, c1, c2, c3] }
    }

    /// Returns the matrix as a flat `[f32; 16]` array suitable for GPU upload.
    pub fn to_cols_array(self) -> [f32; 16] {
        let c: [[f32; 4]; 4] = self.cols;
        [
            c[0][0], c[0][1], c[0][2], c[0][3], c[1][0], c[1][1], c[1][2], c[1][3], c[2][0], c[2][1], c[2][2], c[2][3],
            c[3][0], c[3][1], c[3][2], c[3][3],
        ]
    }

    /// Constructs an orthographic projection matrix for 2D rendering.
    ///
    /// Maps the rectangle `[left, right] × [bottom, top]` to NDC `[-1, 1]²`.
    /// Depth range `[near, far]` maps to `[0, 1]` (WGPU convention).
    ///
    /// # Usage
    /// For a window of size `(w, h)` with the origin at the top-left corner:
    /// ```ignore
    /// Mat4::orthographic(0.0, w, h, 0.0, -1.0, 1.0)
    /// ```
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let rml: f32 = right - left;
        let tmb: f32 = top - bottom;
        let fmn: f32 = far - near;

        Self::from_cols(
            [2.0 / rml, 0.0, 0.0, 0.0],
            [0.0, 2.0 / tmb, 0.0, 0.0],
            [0.0, 0.0, 1.0 / fmn, 0.0],
            [-(right + left) / rml, -(top + bottom) / tmb, -near / fmn, 1.0],
        )
    }

    /// Returns a translation matrix that moves points by `(tx, ty, tz)`.
    pub fn translate(tx: f32, ty: f32, tz: f32) -> Self {
        let mut m: Mat4 = Self::IDENTITY;
        m.cols[3] = [tx, ty, tz, 1.0];
        m
    }

    /// Returns a uniform scale matrix.
    pub fn scale(sx: f32, sy: f32, sz: f32) -> Self {
        let mut m: Mat4 = Self::IDENTITY;
        m.cols[0][0] = sx;
        m.cols[1][1] = sy;
        m.cols[2][2] = sz;
        m
    }

    /// Returns a 2D rotation matrix (rotation around the Z axis).
    pub fn rotate_z(radians: f32) -> Self {
        let (sin, cos): (f32, f32) = radians.sin_cos();
        let mut m: Mat4 = Self::IDENTITY;
        m.cols[0][0] = cos;
        m.cols[0][1] = sin;
        m.cols[1][0] = -sin;
        m.cols[1][1] = cos;
        m
    }

    /// Matrix × matrix multiplication (self × rhs).
    pub fn mul_mat4(self, rhs: Self) -> Self {
        let a: [[f32; 4]; 4] = self.cols;
        let b: [[f32; 4]; 4] = rhs.cols;
        let mut out: [[f32; 4]; 4] = [[0.0f32; 4]; 4];

        for col in 0..4 {
            for row in 0..4 {
                out[col][row] =
                    a[0][row] * b[col][0] + a[1][row] * b[col][1] + a[2][row] * b[col][2] + a[3][row] * b[col][3];
            }
        }

        Self { cols: out }
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl std::ops::Mul for Mat4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.mul_mat4(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-5;

    fn approx_eq(a: [f32; 16], b: [f32; 16]) -> bool {
        a.iter().zip(b.iter()).all(|(x, y): (&f32, &f32)| (*x - *y).abs() < EPS)
    }

    #[test]
    fn identity_mul() {
        let m: Mat4 = Mat4::IDENTITY * Mat4::IDENTITY;
        assert!(approx_eq(m.to_cols_array(), Mat4::IDENTITY.to_cols_array()));
    }

    #[test]
    fn orthographic_maps_corners() {
        let proj: Mat4 = Mat4::orthographic(0.0, 800.0, 600.0, 0.0, -1.0, 1.0);
        let c: [[f32; 4]; 4] = proj.cols;

        assert!((c[0][0] - 2.0 / 800.0).abs() < EPS);
        assert!((c[1][1] - 2.0 / (-600.0_f32)).abs() < EPS);
    }

    #[test]
    fn scale_and_translate_compose() {
        let t: Mat4 = Mat4::translate(5.0, 3.0, 0.0);
        let s: Mat4 = Mat4::scale(2.0, 2.0, 1.0);
        let _m: Mat4 = t * s;
    }
}
