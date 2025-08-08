use crate::number::Number;

impl Number {
    pub fn pow(self, other: &Self) -> Self {
        Self::from_f64(self.0.powf(other.0))
    }

    /// Calculates the factorial of (the floor of) the given `f64`.
    ///
    /// It is the caller's responsibility to ensure that the given `f64` is a
    /// non-negative integer.
    pub fn factorial(self, order: u16) -> Self {
        let mut x = self.0;
        debug_assert!(self.0 >= 0.0);
        debug_assert!(order >= 1);
        x = x.floor();
        let mut result = 1f64;
        while x >= 1. && result != f64::INFINITY {
            result *= x;
            x -= order as f64;
        }
        result.into()
    }

    pub fn abs(self) -> Self {
        Self::from_f64(self.0.abs())
    }

    pub fn rem_euclid(self, rhs: &Self) -> Self {
        Self(self.0.rem_euclid(rhs.0))
    }

    pub fn round(self) -> Self {
        Self(self.0.round())
    }

    pub fn floor(self) -> Self {
        Self(self.0.floor())
    }

    pub fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    pub fn trunc(self) -> Self {
        Self(self.0.trunc())
    }

    pub fn fract(self) -> Self {
        Self(self.0.fract())
    }

    pub fn sin(self) -> Self {
        Self(self.0.sin())
    }

    pub fn cos(self) -> Self {
        Self(self.0.cos())
    }

    pub fn tan(self) -> Self {
        Self(self.0.tan())
    }

    pub fn asin(self) -> Self {
        Self(self.0.asin())
    }

    pub fn acos(self) -> Self {
        Self(self.0.acos())
    }

    pub fn atan(self) -> Self {
        Self(self.0.atan())
    }

    pub fn sinh(self) -> Self {
        Self(self.0.sinh())
    }

    pub fn cosh(self) -> Self {
        Self(self.0.cosh())
    }

    pub fn tanh(self) -> Self {
        Self(self.0.tanh())
    }

    pub fn asinh(self) -> Self {
        Self(self.0.asinh())
    }

    pub fn acosh(self) -> Self {
        Self(self.0.acosh())
    }

    pub fn atanh(self) -> Self {
        Self(self.0.atanh())
    }

    pub fn exp(self) -> Self {
        Self(self.0.exp())
    }

    pub fn ln(self) -> Self {
        Self(self.0.ln())
    }

    pub fn log10(self) -> Self {
        Self(self.0.log10())
    }

    pub fn log2(self) -> Self {
        Self(self.0.log2())
    }

    pub fn gamma(self) -> Self {
        Self(crate::gamma::gamma(self.0))
    }

    pub fn atan2(self, other: &Self) -> Self {
        Self(self.0.atan2(other.0))
    }

    pub fn random() -> Self {
        Self(rand::random::<f64>())
    }
}
