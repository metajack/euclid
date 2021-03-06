use std::cmp::FuzzyEq;
use core::num::{NumCast, One, Zero, cast};

pub pure fn Matrix4<T:Add<T,T> + Copy + FuzzyEq<T> + Mul<T,T> + One + Zero>(
        m11: T, m12: T, m13: T, m14: T,
        m21: T, m22: T, m23: T, m24: T,
        m31: T, m32: T, m33: T, m34: T,
        m41: T, m42: T, m43: T, m44: T)
     -> Matrix4<T> {
    Matrix4 {
        m11: m11, m12: m12, m13: m13, m14: m14,
        m21: m21, m22: m22, m23: m23, m24: m24,
        m31: m31, m32: m32, m33: m33, m34: m34,
        m41: m41, m42: m42, m43: m43, m44: m44
    }
}

pub struct Matrix4<T> {
    m11: T, m12: T, m13: T, m14: T,
    m21: T, m22: T, m23: T, m24: T,
    m31: T, m32: T, m33: T, m34: T,
    m41: T, m42: T, m43: T, m44: T,
}

pub impl<T:Add<T,T> + Copy + FuzzyEq<T> + Mul<T,T> + One + Zero> Matrix4<T> {
    pure fn fuzzy_eq(&self, other: &Matrix4<T>) -> bool {
        self.m11.fuzzy_eq(&other.m11) && self.m12.fuzzy_eq(&other.m12) &&
        self.m13.fuzzy_eq(&other.m13) && self.m14.fuzzy_eq(&other.m14) &&
        self.m21.fuzzy_eq(&other.m21) && self.m22.fuzzy_eq(&other.m22) &&
        self.m23.fuzzy_eq(&other.m23) && self.m24.fuzzy_eq(&other.m24) &&
        self.m31.fuzzy_eq(&other.m31) && self.m32.fuzzy_eq(&other.m32) &&
        self.m33.fuzzy_eq(&other.m33) && self.m34.fuzzy_eq(&other.m34) &&
        self.m41.fuzzy_eq(&other.m41) && self.m42.fuzzy_eq(&other.m42) &&
        self.m43.fuzzy_eq(&other.m43) && self.m44.fuzzy_eq(&other.m44)
    }

    pure fn mul(&self, m: &Matrix4<T>) -> Matrix4<T> {
        Matrix4(m.m11*self.m11 + m.m12*self.m21 + m.m13*self.m31 + m.m14*self.m41,
                m.m11*self.m12 + m.m12*self.m22 + m.m13*self.m32 + m.m14*self.m42,
                m.m11*self.m13 + m.m12*self.m23 + m.m13*self.m33 + m.m14*self.m43,
                m.m11*self.m14 + m.m12*self.m24 + m.m13*self.m34 + m.m14*self.m44,
                m.m21*self.m11 + m.m22*self.m21 + m.m23*self.m31 + m.m24*self.m41,
                m.m21*self.m12 + m.m22*self.m22 + m.m23*self.m32 + m.m24*self.m42,
                m.m21*self.m13 + m.m22*self.m23 + m.m23*self.m33 + m.m24*self.m43,
                m.m21*self.m14 + m.m22*self.m24 + m.m23*self.m34 + m.m24*self.m44,
                m.m31*self.m11 + m.m32*self.m21 + m.m33*self.m31 + m.m34*self.m41,
                m.m31*self.m12 + m.m32*self.m22 + m.m33*self.m32 + m.m34*self.m42,
                m.m31*self.m13 + m.m32*self.m23 + m.m33*self.m33 + m.m34*self.m43,
                m.m31*self.m14 + m.m32*self.m24 + m.m33*self.m34 + m.m34*self.m44,
                m.m41*self.m11 + m.m42*self.m21 + m.m43*self.m31 + m.m44*self.m41,
                m.m41*self.m12 + m.m42*self.m22 + m.m43*self.m32 + m.m44*self.m42,
                m.m41*self.m13 + m.m42*self.m23 + m.m43*self.m33 + m.m44*self.m43,
                m.m41*self.m14 + m.m42*self.m24 + m.m43*self.m34 + m.m44*self.m44)
    }

    pure fn mul_s(&self, x: T) -> Matrix4<T> {
        Matrix4(self.m11 * x, self.m12 * x, self.m13 * x, self.m14 * x,
                self.m21 * x, self.m22 * x, self.m23 * x, self.m24 * x,
                self.m31 * x, self.m32 * x, self.m33 * x, self.m34 * x,
                self.m41 * x, self.m42 * x, self.m43 * x, self.m44 * x)
    }

    pure fn scale(&self, x: T, y: T, z: T) -> Matrix4<T> {
        Matrix4(self.m11 * x, self.m12,     self.m13,     self.m14,
                self.m21,     self.m22 * y, self.m23,     self.m24,
                self.m31,     self.m32,     self.m33 * z, self.m34,
                self.m41,     self.m42,     self.m43,     self.m44)
    }

    pure fn to_array(&self) -> [T * 16] {
        [
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44
        ]
    }

    pure fn translate(&self, x: T, y: T, z: T) -> Matrix4<T> {
        let (_0, _1) = (Zero::zero(), One::one());
        let matrix = Matrix4(_1, _0, _0, _0,
                             _0, _1, _0, _0,
                             _0, _0, _1, _0,
                             x,  y,  z,  _1);

        return self.mul(&matrix);
    }
}

pub fn ortho<T:Add<T,T> + Copy + Div<T,T> + FuzzyEq<T> + Mul<T,T> + Neg<T> + NumCast + One +
               Sub<T,T> + Zero>
        (left: T,
         right: T,
         bottom: T,
         top: T,
         near: T,
         far: T)
      -> Matrix4<T> {
    let _2: T = num::cast(2);
    let _1: T = One::one();
    let _0: T = Zero::zero();

    let tx = -((right + left) / (right - left));
    let ty = -((top + bottom) / (top - bottom));
    let tz = -((far + near) / (far - near));

    Matrix4(_2 / (right - left), _0,                  _0,                 _0,
            _0,                  _2 / (top - bottom), _0,                 _0,
            _0,                  _0,                  -_2 / (far - near), _0,
            tx,                  ty,                  tz,                 _1)
}

pub fn identity<T:Add<T,T> + Copy + FuzzyEq<T> + Mul<T,T> + One + Zero>() -> Matrix4<T> {
    let (_0, _1) = (Zero::zero(), One::one());
    Matrix4(_1, _0, _0, _0,
            _0, _1, _0, _0,
            _0, _0, _1, _0,
            _0, _0, _0, _1)
}

#[test]
pub fn test_ortho() {
    let (left, right, bottom, top) = (0.0, 1.0, 0.1, 1.0);
    let (near, far) = (-1.0, 1.0);
    let result = ortho(left, right, bottom, top, near, far);
    let expected = Matrix4(2.0,  0.0,         0.0,  0.0,
                           0.0,  2.22222222,  0.0,  0.0,
                           0.0,  0.0,         -1.0, 0.0,
                           -1.0, -1.22222222, -0.0, 1.0);
    debug!("result=%? expected=%?", result, expected);
    fail_unless!(result.fuzzy_eq(&expected));
}

