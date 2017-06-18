use std::ops::Mul;

/// A 4x4 matrix mathematics structure
#[derive(Copy,Clone,PartialEq)]
pub struct Mat4 {
    data: [f32; 16]
}

/// Construct methods for Mat4
impl Mat4 {
    /// Constructs a new Mat4 and sets it to the identity matrix
    pub fn identity() -> Self {
        Mat4 {

            //Matrix Mathmatics.
            data: [1.0, 0.0, 0.0, 0.0,
                   0.0, 1.0, 0.0, 0.0,
                   0.0, 0.0, 1.0, 0.0,
                   0.0, 0.0, 0.0, 1.0]
        }
    }
    /// Constructs a new Mat4 and sets it to identity
    pub fn new() -> Self {
        Self::identity()
    }

    /// Construct a new Mat4 from a 16 element array of f32
    pub fn from_array(a: [f32; 16]) -> Self {
        Mat4 {
            data: a
        }
    }
}

/// Get methods for Mat4
impl Mat4 {
    /// Returns the Mat4 as an array of 16 f32
    pub fn as_array(&self) -> [f32; 16] {
        self.data
    }
}

/// Mutate methods for Mat4
impl Mat4 {
    /// Translate Mat4 by a Vec3
    pub fn translate(&mut self, v: super::vec3::Vec3<f32>) {
        let &m = &self.data;
        self.data[12] += m[ 0]*v.x + m[ 4]*v.y + m[ 8]*v.z;
        self.data[13] += m[ 1]*v.x + m[ 5]*v.y + m[ 9]*v.z;
        self.data[14] += m[ 2]*v.x + m[ 6]*v.y + m[10]*v.z;
    }
    /// Rotate Mat4 around the x axis by angle in degrees
    pub fn rotate_x(&mut self, angle: f32) {
        let (c, s, mut t);

        let m = &mut self.data;

        c = angle.to_radians().cos();
        s = angle.to_radians().sin();

        t = m[ 4];
        m[ 4] = t*c  + m[ 8]*s;
        m[ 8] = t*-s + m[ 8]*c;

        t = m[ 5];
        m[ 5] = t*c  + m[ 9]*s;
        m[ 9] = t*-s + m[ 9]*c;

        t = m[ 6];
        m[ 6] = t*c  + m[10]*s;
        m[10] = t*-s + m[10]*c;
    }
    /// Rotate Mat4 around the y axis by angle in degrees
    pub fn rotate_y(&mut self, angle: f32) {
        let (c, s, mut t);

        let m = &mut self.data;

        c = angle.to_radians().cos();
        s = angle.to_radians().sin();

        t = m[ 0];
        m[ 0] = t*c  + m[ 8]*s;
        m[ 8] = t*-s + m[ 8]*c;

        t = m[ 1];
        m[ 1] = t*c  + m[ 9]*s;
        m[ 9] = t*-s + m[ 9]*c;

        t = m[ 2];
        m[ 2] = t*c  + m[10]*s;
        m[10] = t*-s + m[10]*c;
    }
    /// Rotate Mat4 around the z axis by angle in degrees
    pub fn rotate_z(&mut self, angle: f32) {
        let (c, s, mut t);

        let m = &mut self.data;

        c = angle.to_radians().cos();
        s = angle.to_radians().sin();

        t = m[ 0];
        m[ 0] = t*c  + m[ 4]*s;
        m[ 4] = t*-s + m[ 4]*c;

        t = m[ 1];
        m[ 1] = t*c  + m[ 5]*s;
        m[ 5] = t*-s + m[ 5]*c;

        t = m[ 2];
        m[ 2] = t*c  + m[ 6]*s;
        m[ 6] = t*-s + m[ 6]*c;
    }
    /// Rotate Mat4 by Vec3<f32> in degrees
    pub fn rotate(&mut self, v: super::vec3::Vec3<f32>) {
        self.rotate_z(v.z);
        self.rotate_x(v.x);
        self.rotate_y(v.y);
    }
    /// Scale Mat4 by Vec3<f32>
    pub fn scale(&mut self, v: super::vec3::Vec3<f32>) {
        let m = &mut self.data;

        m[0] *= v.x; m[4] *= v.x; m[ 8] *= v.x;
        m[1] *= v.y; m[5] *= v.y; m[ 9] *= v.y;
        m[2] *= v.z; m[6] *= v.z; m[10] *= v.z;
    }

    /// Set Mat4 translation
    pub fn set_translation(&mut self, v: super::vec3::Vec3<f32>) {
        self.data[12] = v.x;
        self.data[13] = v.y;
        self.data[14] = v.z;
    }
    /// Set Mat4 rotation in degrees
    pub fn set_rotation(&mut self, v: super::vec3::Vec3<f32>) {
        let (x,y,z) = (v.x.to_radians(), v.y.to_radians(), v.z.to_radians());
        let (sx,sy,sz, cx,cy,cz);
        let cz_nsx;

        sx=x.sin(); sy=y.sin(); sz=z.sin();
        cx=x.cos(); cy=y.cos(); cz=z.cos();

        cz_nsx = cz*-sx;

        let m = &mut self.data;

        m[ 0] = cz*cy;             m[ 4] = -sz*cx; m[ 8] = cz*-sy + sz*sx*cy;
        m[ 1] = sz*cy + cz_nsx*sy; m[ 5] =  cz*cx; m[ 9] = sz*-sy + cz_nsx*cy;
        m[ 2] = cx*sy;             m[ 6] =  sx;    m[10] = cx*cy;
    }
    /// Set Mat4 scaling
    pub fn set_scaling(&mut self, v: super::vec3::Vec3<f32>) {
        self.data[ 0] = v.x;
        self.data[ 5] = v.y;
        self.data[10] = v.z;
    }
    /// Set Mat4 orthographic projection
    pub fn set_orthographic(&mut self, left: f32, right: f32, bottom: f32,
                            top: f32, z_near: f32, z_far: f32) {
        let (a,b,c,d,e,f);

        a =             2.0/(right  - left);
        b = (left + right )/(left   - right);
        c =             2.0/(top    - bottom);
        d = (top  + bottom)/(bottom - top);
        e =             1.0/(z_far  - z_near);
        f =          z_near/(z_near - z_far);

        self.data[ 0] = a;
        self.data[12] = b;
        self.data[ 5] = c;
        self.data[13] = d;
        self.data[10] = e;
        self.data[14] = f;
    }
    /// Set Mat4 perspective projection
    pub fn set_perspective(&mut self, fov: super::Angle<f32>, aspect: f32, z_near: f32,
                           z_far: f32) {
        let (a,b,c,d);

        b = 1f32/(fov*0.5).to_radians().tan();
        a = b/aspect;

        c =         z_far/(z_far - z_near);
        d = -z_near*z_far/(z_far - z_near);

        self.data[ 0] = a;
        self.data[ 5] = b;
        self.data[10] = c;
        self.data[14] = d;
    }
}

impl Mul for Mat4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let &a = &self.data;
        let &b = &rhs.data;
        Mat4 {
            data: [
                //COLUMN MAJOR
                a[ 0]*b[ 0] + a[ 1]*b[ 4] + a[ 2]*b[ 8] + a[ 3]*b[12],
                a[ 0]*b[ 1] + a[ 1]*b[ 5] + a[ 2]*b[ 9] + a[ 3]*b[13],
                a[ 0]*b[ 2] + a[ 1]*b[ 6] + a[ 2]*b[10] + a[ 3]*b[14],
                a[ 0]*b[ 3] + a[ 1]*b[ 7] + a[ 2]*b[11] + a[ 3]*b[15],

                a[ 4]*b[ 0] + a[ 5]*b[ 4] + a[ 6]*b[ 8] + a[ 7]*b[12],
                a[ 4]*b[ 1] + a[ 5]*b[ 5] + a[ 6]*b[ 9] + a[ 7]*b[13],
                a[ 4]*b[ 2] + a[ 5]*b[ 6] + a[ 6]*b[10] + a[ 7]*b[14],
                a[ 4]*b[ 3] + a[ 5]*b[ 7] + a[ 6]*b[11] + a[ 7]*b[15],

                a[ 8]*b[ 0] + a[ 9]*b[ 4] + a[10]*b[ 8] + a[11]*b[12],
                a[ 8]*b[ 1] + a[ 9]*b[ 5] + a[10]*b[ 9] + a[11]*b[13],
                a[ 8]*b[ 2] + a[ 9]*b[ 6] + a[10]*b[10] + a[11]*b[14],
                a[ 8]*b[ 3] + a[ 9]*b[ 7] + a[10]*b[11] + a[11]*b[15],

                a[12]*b[ 0] + a[13]*b[ 4] + a[14]*b[ 8] + a[15]*b[12],
                a[12]*b[ 1] + a[13]*b[ 5] + a[14]*b[ 9] + a[15]*b[13],
                a[12]*b[ 2] + a[13]*b[ 6] + a[14]*b[10] + a[15]*b[14],
                a[12]*b[ 3] + a[13]*b[ 7] + a[14]*b[11] + a[15]*b[15]

                /*ROW MAJOR
                a[ 0]*b[ 0] + a[ 4]*b[ 1] + a[ 8]*b[ 2] + a[12]*b[ 3],
                a[ 1]*b[ 0] + a[ 5]*b[ 1] + a[ 9]*b[ 2] + a[13]*b[ 3],
                a[ 2]*b[ 0] + a[ 6]*b[ 1] + a[10]*b[ 2] + a[14]*b[ 3],
                a[ 3]*b[ 0] + a[ 7]*b[ 1] + a[11]*b[ 2] + a[15]*b[ 3],
                a[ 0]*b[ 4] + a[ 4]*b[ 5] + a[ 8]*b[ 6] + a[12]*b[ 7],
                a[ 1]*b[ 4] + a[ 5]*b[ 5] + a[ 9]*b[ 6] + a[13]*b[ 7],
                a[ 2]*b[ 4] + a[ 6]*b[ 5] + a[10]*b[ 6] + a[14]*b[ 7],
                a[ 3]*b[ 4] + a[ 7]*b[ 5] + a[11]*b[ 6] + a[15]*b[ 7],
                a[ 0]*b[ 8] + a[ 4]*b[ 9] + a[ 8]*b[10] + a[12]*b[11],
                a[ 1]*b[ 8] + a[ 5]*b[ 9] + a[ 9]*b[10] + a[13]*b[11],
                a[ 2]*b[ 8] + a[ 6]*b[ 9] + a[10]*b[10] + a[14]*b[11],
                a[ 3]*b[ 8] + a[ 7]*b[ 9] + a[11]*b[10] + a[15]*b[11],
                a[ 0]*b[12] + a[ 4]*b[13] + a[ 8]*b[14] + a[12]*b[15],
                a[ 1]*b[12] + a[ 5]*b[13] + a[ 9]*b[14] + a[13]*b[15],
                a[ 2]*b[12] + a[ 6]*b[13] + a[10]*b[14] + a[14]*b[15],
                a[ 3]*b[12] + a[ 7]*b[13] + a[11]*b[14] + a[15]*b[15]*/
            ]
        }
    }
}
impl Mul<super::vec3::Vec3<f32>> for Mat4 {
    type Output = super::vec3::Vec3<f32>;

    fn mul(self, rhs: super::vec3::Vec3<f32>) -> super::vec3::Vec3<f32> {
        super::vec3::Vec3 {
            x: self.data[0]*rhs.x + self.data[4]*rhs.y + self.data[ 8]*rhs.z,
            y: self.data[1]*rhs.x + self.data[5]*rhs.y + self.data[ 9]*rhs.z,
            z: self.data[2]*rhs.x + self.data[6]*rhs.y + self.data[10]*rhs.z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Mat4;
    use super::super::vec3::Vec3;

    #[test]
    fn matrix_multiply() {
        let a_data = [
            1.0, 5.0, 9.0, 13.0,
            2.0, 6.0, 10.0, 14.0,
            3.0, 7.0, 11.0, 15.0,
            4.0, 8.0, 12.0, 16.0f32
        ];
        let b_data = [
            2.0, 2.0, 2.0, 2.0,
            2.0, 2.0, 2.0, 2.0,
            2.0, 2.0, 2.0, 2.0,
            2.0, 2.0, 2.0, 2.0f32
        ];

        let a = Mat4::from_array(a_data);
        let b = Mat4::from_array(b_data);

        let r = a * b;

        let arr = r.as_array();

        println!("Layout:");
        for i in 0..4 {
            print!("| ");
            for j in 0..4 {
                print!("{} ", j*4 + i);
            }
            println!("|");
        }

        println!("Result matrix:");
        for i in 0..4 {
            print!("| ");
            for j in 0..4 {
                print!("{} ", arr[j*4 + i]);
            }
            println!("|");
        }

        assert!(arr[ 0] == arr[ 1] && arr[ 1] == arr[ 2] && arr[ 2] == arr[ 3]);
        assert!(arr[ 4] == arr[ 5] && arr[ 5] == arr[ 6] && arr[ 6] == arr[ 7]);
        assert!(arr[ 8] == arr[ 9] && arr[ 9] == arr[10] && arr[10] == arr[11]);
        assert!(arr[12] == arr[13] && arr[13] == arr[14] && arr[14] == arr[15]);

        assert!((arr[ 3] - 56.0).abs() < 0.00001);
        assert!((arr[ 5] - 64.0).abs() < 0.00001);
        assert!((arr[10] - 72.0).abs() < 0.00001);
        assert!((arr[12] - 80.0).abs() < 0.00001);
    }
    #[test]
    fn vec3_multiply() {
        let mut m = Mat4::new();

        m.rotate_y(90f32);

        let r = m * Vec3::from(32.0, 0.0, 42.0);

        println!("r = <{}, {}, {}>", r.x, r.y, r.z);

        assert!((r.x - -42.0).abs() < 0.00001);
        assert!((r.z -  32.0).abs() < 0.00001);
    }
    #[test]
    fn translate_rotate_z() {
        let mut m = Mat4::new();

        // set rotation around z axis of 90 degrees
        m.rotate_z(90f32);

        // translate by 5 on x
        m.translate(Vec3::from(5.0, 0.0, 0.0));

        // we should now have a rotated translation of <0.0, 5.0, 0.0>

        let arr = m.as_array();

        println!("result pos: <{}, {}, {}>", arr[12], arr[13], arr[14]);

        // check x is ~zero
        assert!((arr[12] - 0.0).abs() < 0.00001);
        // check y is ~5.0
        assert!((arr[13] - 5.0).abs() < 0.00001);
    }
    #[test]
    fn translate_rotate_x() {
        let mut m = Mat4::new();

        // set rotation around x axis of 90 degrees
        m.rotate_x(90f32);

        // translate by 5 on x
        m.translate(Vec3::from(0.0, 5.0, 0.0));

        // we should now have a rotated translation of <0.0, 0.0, 5.0>

        let arr = m.as_array();

        println!("result pos: <{}, {}, {}>", arr[12], arr[13], arr[14]);

        // check y is ~zero
        assert!((arr[12] - 0.0).abs() < 0.00001);
        // check z is ~5.0
        assert!((arr[14] - 5.0).abs() < 0.00001);
    }
    #[test]
    fn translate_rotate_y() {
        let mut m = Mat4::new();

        // set rotation around y axis of 90 degrees
        m.rotate_y(90f32);

        // translate by 5 on z
        m.translate(Vec3::from(0.0, 0.0, 5.0));

        // we should now have a rotated translation of <-5.0, 0.0, 0.0>

        let arr = m.as_array();

        println!("result pos: <{}, {}, {}>", arr[12], arr[13], arr[14]);

        // check x is ~-5.0
        assert!((arr[12] - -5.0).abs() < 0.00001);
        // check y is ~zero
        assert!((arr[13] -  0.0).abs() < 0.00001);
    }
}
