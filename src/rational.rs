use std::{fmt, ops};

use crate::bigint::BigInt;

#[derive(Debug, Clone)]
pub struct Rational {
    numerator: BigInt,
    denominator: BigInt
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.denominator == BigInt::from(1) {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "({} / {})", self.numerator, self.denominator)
        }
    }
}

impl Rational {
    pub fn zero() -> Rational {
        Rational::from(0)
    }
    
    pub fn power(&self, exp: u32) -> Rational {
        if exp == 0 {
            return Rational::from(1);
        }

        let mut result = self.clone();
        for _v in 2..=exp {
            result = &result * self;
        }
        result
    }

    pub fn abs(&self) -> Rational {
        if self >= &Rational::zero() {
            self.clone()
        } else {
            -self
        }
    }
}

impl From<i32> for Rational {
    fn from(value: i32) -> Self {
        Rational::from((value, 1))
    }
}

impl From<(i32, i32)> for Rational {
    fn from(value: (i32, i32)) -> Self {
        Rational::from((BigInt::from(value.0), BigInt::from(value.1)))
    }
}

impl From<BigInt> for Rational {
    fn from(value: BigInt) -> Self {
        Rational::from((value, BigInt::from(1)))
    }
}

impl From<(BigInt, BigInt)> for Rational {
    fn from(value: (BigInt, BigInt)) -> Self {
        Rational { numerator: value.0, denominator: value.1 }
    }
}

impl ops::Add<&Rational> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Rational {
        let (numerator, denominator) = if self.denominator == rhs.denominator {
            (&self.numerator + &rhs.numerator, self.denominator.clone())
        } else {
            (&self.numerator * &rhs.denominator + &rhs.numerator * &self.denominator,
                &self.denominator * &rhs.denominator)
        };

        Rational { numerator, denominator }
    }
}

impl ops::Add<Rational> for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Rational {
        &self + &rhs
    }
}


impl ops::AddAssign<Rational> for Rational {
    fn add_assign(&mut self, rhs: Rational) {
        *self = &*self + &rhs;
    }
}

impl ops::Neg for &Rational {
    type Output = Rational;
    fn neg(self) -> Self::Output {
        &Rational::from(-1) * self
    }
}

impl ops::Neg for Rational {
    type Output = Rational;
    fn neg(self) -> Self::Output {
        Rational::from(-1) * self
    }
}

impl ops::Sub<&Rational> for &Rational {
    type Output = Rational;
    fn sub(self, rhs: &Rational) -> Self::Output {
        self + &(-rhs)
    }
}

impl ops::Sub<Rational> for Rational {
    type Output = Rational;
    fn sub(self, rhs: Rational) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::SubAssign<Rational> for Rational {
    fn sub_assign(&mut self, rhs: Rational) {
        *self = &*self - &rhs;
    }
}

impl ops::Mul<&Rational> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        let numerator = &self.numerator * &rhs.numerator;
        let denominator = &self.denominator * &rhs.denominator;

        if denominator >= BigInt::zero() {
            Rational { numerator, denominator }
        } else {
            Rational { numerator: -numerator, denominator: -denominator }
        }
    }
}

impl ops::Mul<Rational> for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        &self * &rhs
    }
}

impl ops::MulAssign<Rational> for Rational {
    fn mul_assign(&mut self, rhs: Rational) {
        *self = &*self * &rhs;
    }
}

impl std::cmp::PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs = &self.numerator * &other.denominator;
        let rhs = &self.denominator * &other.numerator;
        let result = lhs.cmp(&rhs);
        result
    }
}

impl std::cmp::PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl std::cmp::Eq for Rational { }
