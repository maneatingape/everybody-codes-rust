use crate::util::integer::*;

pub trait MathOps<T: Integer> {
    #[must_use]
    fn gcd(self, b: T) -> T;
    #[must_use]
    fn lcm(self, b: T) -> T;
    #[must_use]
    fn mod_pow(self, e: T, m: T) -> T;
}

impl<T: Integer> MathOps<T> for T {
    #[inline]
    fn gcd(self, mut b: T) -> T {
        let mut a = self;

        while b != T::ZERO {
            (a, b) = (b, a % b);
        }

        a
    }

    #[inline]
    fn lcm(self, b: T) -> T {
        self * (b / self.gcd(b))
    }

    #[inline]
    fn mod_pow(self, mut e: T, m: T) -> T {
        let mut base = self;
        let mut result = T::ONE;

        while e > T::ZERO {
            if e & T::ONE == T::ONE {
                result = (result * base) % m;
            }
            base = (base * base) % m;
            e = e >> 1;
        }

        result
    }
}
