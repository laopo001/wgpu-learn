use cgmath::prelude::*;
use std::f32::consts::PI;
pub const DEG_TO_RAD: f32 = PI / 180.0;
pub const RAD_TO_DEG: f32 = 180.0 / PI;

use crate::Matrix4 as Mat4;
use crate::Point3;
use crate::Quat;
use crate::Vector3 as Vec3;
use crate::Vector4 as Vec4;
// use cgmath::prelude::SquareMatrix;
// use cgmath::Array;
pub trait Into<T>: Sized {
    fn into2(self) -> T;
}

pub trait Matrix4Plus {
    fn get_translate(&self) -> Vec3;
    fn get(&self, i: usize) -> f32;
    fn data(&self) -> Vec<f32>;
    fn get_euler_angles(&self, eulers: &mut Vec3);
    fn get_scale(&self, v: &mut Vec3);
    fn get_x(&self, v: &mut Vec3);
    fn get_y(&self, v: &mut Vec3);
    fn get_z(&self, v: &mut Vec3);
    fn set_from_trs(&mut self, t: &Vec3, r: &Quat, s: &Vec3);
    fn copy(&mut self, v: &Mat4);
    fn set(
        &mut self,
        n0: f32,
        n1: f32,
        n2: f32,
        n3: f32,
        n4: f32,
        n5: f32,
        n6: f32,
        n7: f32,
        n8: f32,
        n9: f32,
        n10: f32,
        n11: f32,
        n12: f32,
        n13: f32,
        n14: f32,
        n15: f32,
    );
}

impl Matrix4Plus for Mat4 {
    fn get_translate(&self) -> Vec3 {
        Vec3::new(self.w.x, self.w.y, self.w.z)
    }
    fn get(&self, i: usize) -> f32 {
        let m: [[f32; 4]; 4] = self.clone().into();
        return m[i / 4][i % 4];
    }
    fn data(&self) -> Vec<f32> {
        let m: [[f32; 4]; 4] = self.clone().into();
        m.concat()
    }

    fn get_scale(&self, v: &mut Vec3) {
        let mut temp1 = Vec3::zero();
        let mut temp2 = Vec3::zero();
        let mut temp3 = Vec3::zero();
        self.get_x(&mut temp1);
        self.get_y(&mut temp2);
        self.get_z(&mut temp3);
        v.set(temp1.length(), temp2.length(), temp3.length());
    }
    fn get_x(&self, v: &mut Vec3) {
        let m = self.data();
        v.set(m[0], m[1], m[2]);
    }

    fn get_y(&self, v: &mut Vec3) {
        let m = self.data();
        v.set(m[4], m[5], m[6]);
    }

    fn get_z(&self, v: &mut Vec3) {
        let m = self.data();
        v.set(m[8], m[9], m[10]);
    }
    fn get_euler_angles(&self, eulers: &mut Vec3) {
        let mut scale = Vec3::zero();
        self.get_scale(&mut scale);
        let sx = scale.x;
        let sy = scale.y;
        let sz = scale.z;

        let m = self.data();
        let x: f32;
        let y = (-m[2] / sx).asin();
        let z: f32;
        let half_pi = std::f32::consts::PI * 0.5;
        if y < half_pi {
            if y > -half_pi {
                x = (m[6] / sy).atan2(m[10] / sz);
                z = (m[1] / sx).atan2(m[0] / sx);
            } else {
                z = 0.0;
                x = -(m[4] / sy).atan2(m[5] / sy);
            }
        } else {
            z = 0.0;
            x = (m[4] / sy).atan2(m[5] / sy);
        }
        eulers.set(x, y, z);
        eulers.scale(RAD_TO_DEG);
    }
    fn set_from_trs(&mut self, t: &Vec3, r: &Quat, s: &Vec3) {
        let tx = t.x;
        let ty = t.y;
        let tz = t.z;

        let qx = r.v.x;
        let qy = r.v.y;
        let qz = r.v.z;
        let qw = r.s;

        let sx = s.x;
        let sy = s.y;
        let sz = s.z;

        let x2 = qx + qx;
        let y2 = qy + qy;
        let z2 = qz + qz;
        let xx = qx * x2;
        let xy = qx * y2;
        let xz = qx * z2;
        let yy = qy * y2;
        let yz = qy * z2;
        let zz = qz * z2;
        let wx = qw * x2;
        let wy = qw * y2;
        let wz = qw * z2;
        let mut m = self.data();
        m[0] = (1.0 - (yy + zz)) * sx;
        m[1] = (xy + wz) * sx;
        m[2] = (xz - wy) * sx;
        m[3] = 0.0;

        m[4] = (xy - wz) * sy;
        m[5] = (1.0 - (xx + zz)) * sy;
        m[6] = (yz + wx) * sy;
        m[7] = 0.0;

        m[8] = (xz + wy) * sz;
        m[9] = (yz - wx) * sz;
        m[10] = (1.0 - (xx + yy)) * sz;
        m[11] = 0.0;

        m[12] = tx;
        m[13] = ty;
        m[14] = tz;
        m[15] = 1.0;
        self.x.x = m[0];
        self.x.y = m[1];
        self.x.z = m[2];
        self.x.w = m[3];

        self.y.x = m[4];
        self.y.y = m[5];
        self.y.z = m[6];
        self.y.w = m[7];

        self.z.x = m[8];
        self.z.y = m[9];
        self.z.z = m[10];
        self.z.w = m[11];

        self.w.x = m[12];
        self.w.y = m[13];
        self.w.z = m[14];
        self.w.w = m[15];
    }
    fn copy(&mut self, v: &Mat4) {
        let data = v.data();
        let n0 = data[0];
        let n1 = data[1];
        let n2 = data[2];
        let n3 = data[3];
        let n4 = data[4];
        let n5 = data[5];
        let n6 = data[6];
        let n7 = data[7];
        let n8 = data[8];
        let n9 = data[9];
        let n10 = data[10];
        let n11 = data[11];
        let n12 = data[12];
        let n13 = data[13];
        let n14 = data[14];
        let n15 = data[15];
        self.set(
            n0, n1, n2, n3, n4, n5, n6, n7, n8, n9, n10, n11, n12, n13, n14, n15,
        );
    }
    fn set(
        &mut self,
        n0: f32,
        n1: f32,
        n2: f32,
        n3: f32,
        n4: f32,
        n5: f32,
        n6: f32,
        n7: f32,
        n8: f32,
        n9: f32,
        n10: f32,
        n11: f32,
        n12: f32,
        n13: f32,
        n14: f32,
        n15: f32,
    ) {
        self.x.x = n0;
        self.x.y = n1;
        self.x.z = n2;
        self.x.w = n3;

        self.y.x = n4;
        self.y.y = n5;
        self.y.z = n6;
        self.y.w = n7;

        self.z.x = n8;
        self.z.y = n9;
        self.z.z = n10;
        self.z.w = n11;

        self.w.x = n12;
        self.w.y = n13;
        self.w.z = n14;
        self.w.w = n15;
    }
}

impl Into<Vec3> for Point3 {
    fn into2(self) -> Vec3 {
        return Vec3::new(self.x, self.y, self.z);
    }
}

pub trait QuatPlus {
    fn set(&mut self, x: f32, y: f32, z: f32, w: f32);
    fn set_from_mat4(&mut self, mat: &Mat4);
    fn set_from_euler_angles(&mut self, ex: f32, ey: f32, ez: f32);
    fn get_euler_angles(&self, eulers: &mut Vec3);
    fn conjugate(&mut self) -> &mut Self;
    fn normalize(&mut self);
    fn length(&self) -> f32;
    fn length_sq(&self) -> f32;
    fn invert(&mut self);
}
impl QuatPlus for Quat {
    fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.v.set(x, y, z);
        self.s = w;
    }
    fn set_from_mat4(&mut self, mat: &Mat4) {
        let m = mat.data();

        let mut m00 = m[0];
        let mut m01 = m[1];
        let mut m02 = m[2];
        let mut m10 = m[4];
        let mut m11 = m[5];
        let mut m12 = m[6];
        let mut m20 = m[8];
        let mut m21 = m[9];
        let mut m22 = m[10];
        let lx = 1.0 / (m00 * m00 + m01 * m01 + m02 * m02).sqrt();
        let ly = 1.0 / (m10 * m10 + m11 * m11 + m12 * m12).sqrt();
        let lz = 1.0 / (m20 * m20 + m21 * m21 + m22 * m22).sqrt();
        m00 *= lx;
        m01 *= lx;
        m02 *= lx;
        m10 *= ly;
        m11 *= ly;
        m12 *= ly;
        m20 *= lz;
        m21 *= lz;
        m22 *= lz;
        let tr = m00 + m11 + m22;
        let mut rs: f32;
        if tr >= 0.0 {
            let mut s = (tr + 1.0).sqrt();
            self.s = s * 0.5;
            s = 0.5 / s;
            self.v.x = (m12 - m21) * s;
            self.v.y = (m20 - m02) * s;
            self.v.z = (m01 - m10) * s;
        } else {
            if m00 > m11 {
                if m00 > m22 {
                    // XDiagDomMatrix
                    rs = (m00 - (m11 + m22)) + 1.0;
                    rs = rs.sqrt();

                    self.v.x = rs * 0.5;
                    rs = 0.5 / rs;
                    self.s = (m12 - m21) * rs;
                    self.v.y = (m01 + m10) * rs;
                    self.v.z = (m02 + m20) * rs;
                } else {
                    // ZDiagDomMatrix
                    rs = (m22 - (m00 + m11)) + 1.0;
                    rs = rs.sqrt();

                    self.v.z = rs * 0.5;
                    rs = 0.5 / rs;
                    self.s = (m01 - m10) * rs;
                    self.v.x = (m20 + m02) * rs;
                    self.v.y = (m21 + m12) * rs;
                }
            } else if m11 > m22 {
                // YDiagDomMatrix
                rs = (m11 - (m22 + m00)) + 1.0;
                rs = rs.sqrt();

                self.v.y = rs * 0.5;
                rs = 0.5 / rs;
                self.s = (m20 - m02) * rs;
                self.v.z = (m12 + m21) * rs;
                self.v.x = (m10 + m01) * rs;
            } else {
                // ZDiagDomMatrix
                rs = (m22 - (m00 + m11)) + 1.0;
                rs = rs.sqrt();

                self.v.z = rs * 0.5;
                rs = 0.5 / rs;
                self.s = (m01 - m10) * rs;
                self.v.x = (m20 + m02) * rs;
                self.v.y = (m21 + m12) * rs;
            }
        }
    }
    fn set_from_euler_angles(&mut self, ex: f32, ey: f32, ez: f32) {
        let hald = 0.5 * DEG_TO_RAD;

        let ex = ex * hald;
        let ey = ey * hald;
        let ez = ez * hald;

        let sx = ex.sin();
        let cx = ex.cos();
        let sy = ey.sin();
        let cy = ey.cos();
        let sz = ez.sin();
        let cz = ez.cos();

        self.v.x = sx * cy * cz - cx * sy * sz;
        self.v.y = cx * sy * cz + sx * cy * sz;
        self.v.z = cx * cy * sz - sx * sy * cz;
        self.s = cx * cy * cz + sx * sy * sz;
    }
    fn get_euler_angles(&self, eulers: &mut Vec3) {
        let qx = self.v.x;

        let qy = self.v.y;
        let qz = self.v.z;
        let qw = self.s;
        let a2 = 2.0 * (qw * qy - qx * qz);
        let x: f32;
        let y: f32;
        let z: f32;
        if a2 <= -0.99999 {
            x = 2.0 * qx.atan2(qw);
            y = -std::f32::consts::PI / 2.0;
            z = 0.0;
        } else if a2 >= 0.99999 {
            x = 2.0 * qx.atan2(qw);
            y = std::f32::consts::PI / 2.0;
            z = 0.0;
        } else {
            x = (2.0 * (qw * qx + qy * qz)).atan2(1.0 - 2.0 * (qx * qx + qy * qy));
            y = a2.asin();
            z = (2.0 * (qw * qz + qx * qy)).atan2(1.0 - 2.0 * (qy * qy + qz * qz));
        }
        eulers.set(x, y, z);
        eulers.scale(RAD_TO_DEG);
    }
    fn conjugate(&mut self) -> &mut Self {
        self.v.x *= -1.0;
        self.v.y *= -1.0;
        self.v.z *= -1.0;
        return self;
    }
    fn normalize(&mut self) {
        let len = self.length();
        if len == 0.0 {
            self.v.x = 0.0;
            self.v.y = 0.0;
            self.v.z = 0.0;
            self.s = 1.0;
            return;
        }
        let inv = 1.0 / len;
        self.v.x *= inv;
        self.v.y *= inv;
        self.v.z *= inv;
        self.s *= inv;
    }
    fn length_sq(&self) -> f32 {
        return self.v.x * self.v.x + self.v.y * self.v.y + self.v.z * self.v.z + self.s * self.s;
    }
    fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }
    fn invert(&mut self) {
        return self.conjugate().normalize();
    }
}

pub trait Vector3Plus {
    fn scale(&mut self, scalar: f32);
    fn length(&self) -> f32;
    fn length_sq(&self) -> f32;
    fn set(&mut self, x: f32, y: f32, z: f32);
    fn data(&self) -> Box<[f32]>;
}
impl Vector3Plus for Vec3 {
    fn scale(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
    fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }
    fn length_sq(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    fn data(&self) -> Box<[f32]> {
        return Box::new([self.x, self.y, self.z]);
    }
}

pub trait Vector4Plus {
    fn set(&mut self, x: f32, y: f32, z: f32, w: f32);
}
impl Vector4Plus for Vec4 {
    fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
}

#[test]
fn mat4_add() {
    let mut mat1 = Mat4::new(
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    );
    let mat2 = Mat4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    );
    mat1 = mat1 + mat2;
    assert_eq!(mat2.data()[0], 1.0);
}
