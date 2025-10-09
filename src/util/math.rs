use crate::util::integer::*;

pub trait IntegerMathOps<T: Integer<T>> {
    fn gcd(self, b: T) -> T;
    fn lcm(self, b: T) -> T;
    fn mod_pow(self, e: T, m: T) -> T;
}

pub trait SignedMathOps<T: Signed<T>> {
    fn mod_inv(self, m: T) -> Option<T>;
}

impl<T: Integer<T>> IntegerMathOps<T> for T {
    /// Greatest common divisor
    #[inline]
    fn gcd(self, mut b: T) -> T {
        let mut a = self;

        while b != T::ZERO {
            (a, b) = (b, a % b);
        }

        a
    }

    /// Least common multiple
    #[inline]
    fn lcm(self, b: T) -> T {
        self * (b / self.gcd(b))
    }

    /// Modular exponentiation
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

impl<T: Signed<T>> SignedMathOps<T> for T {
    /// Modular multiplicative inverse
    #[inline]
    fn mod_inv(self, m: T) -> Option<T> {
        let mut t = T::ZERO;
        let mut new_t = T::ONE;
        let mut r = m;
        let mut new_r = self;

        while new_r != T::ZERO {
            let quotient = r / new_r;
            (t, new_t) = (new_t, t - quotient * new_t);
            (r, new_r) = (new_r, r - quotient * new_r);
        }

        if r > T::ONE {
            return None;
        }
        if t < T::ZERO {
            t = t + m;
        }

        Some(t)
    }
}
