#![allow(dead_code)]
use rand::Rng;

/// This is an implementation of finite field of prime order (3 * 2 ** 30 + 1)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldElement {
    // u32 is not enough to store the result of multiplication
    val: u64,
}

impl FieldElement {
    const MODULUS: u64 = 3 * (1 << 30) + 1;
    const GENERATOR: u64 = 5;

    pub fn new(val: u64) -> Self {
        FieldElement {
            val: val % Self::MODULUS,
        }
    }

    pub fn zero() -> Self {
        FieldElement::new(0)
    }

    pub fn one() -> Self {
        FieldElement::new(1)
    }

    pub fn generator() -> Self {
        FieldElement::new(Self::GENERATOR)
    }

    pub fn random_element(exclude_elements: &[Self]) -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let val = rng.gen_range(0..Self::MODULUS);
            let element = FieldElement::new(val);
            if !exclude_elements.contains(&element) {
                return element;
            }
        }
    }

    pub fn negate(&self) -> Self {
        FieldElement::new(Self::MODULUS - self.val)
    }

    pub fn add(&self, other: &Self) -> Self {
        FieldElement::new(self.val + other.val)
    }

    pub fn sub(&self, other: &Self) -> Self {
        FieldElement::new(self.val + Self::MODULUS - other.val)
    }

    pub fn mul(&self, other: &Self) -> Self {
        FieldElement::new(self.val * other.val)
    }

    pub fn div(&self, other: &Self) -> Self {
        self.mul(&other.inverse())
    }

    // This is the implementation of the extended Euclidean algorithm
    // ax + by = gcd(a, b) = 1 (mod MODULUS)
    // if a = MODULUS and b = self , y = b ^ (-1) (mod MODULUS)
    // x1 = 1, y1 = 0, x2 = 0, y2 = 1
    pub fn inverse(&self) -> Self {
        let mut a = Self::MODULUS;
        let mut b = self.val;
        let mut x1 = 1;
        let mut y1 = 0;
        let mut x2 = 0;
        let mut y2 = 1;
        while b != 0 {
            let q = a / b;
            let r = a % b;
            let x = x1 - q * x2;
            let y = y1 - q * y2;
            a = b;
            b = r;
            x1 = x2;
            x2 = x;
            y1 = y2;
            y2 = y;
        }
        FieldElement::new(y1)
    }

    pub fn pow(&self, exp: u64) -> Self {
        let mut result = FieldElement::one();
        let mut base = *self;
        let mut exp = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            exp /= 2;
        }
        result
    }

    pub fn is_order(&self, order: u64) -> bool {
        // 有限域元素的阶都是正整数
        assert!(order > 0);

        let mut base = FieldElement::one();
        for _ in 1..order {
            base = base.mul(self);
            if base == FieldElement::one() {
                return false;
            }
        }
        base.mul(self) == FieldElement::one()
    }
}

impl std::fmt::Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl std::ops::Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        FieldElement::new((self.val + other.val) % Self::MODULUS)
    }
}

impl std::ops::Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        FieldElement::new((self.val + Self::MODULUS - other.val) % Self::MODULUS)
    }
}

impl std::ops::Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        FieldElement::new((self.val * other.val) % Self::MODULUS)
    }
}

impl std::ops::Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.mul(&other.inverse())
    }
}

impl std::ops::Neg for FieldElement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        FieldElement::new((Self::MODULUS - self.val) % Self::MODULUS)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use hex::ToHex;
    use sha2::{Digest, Sha256};

    #[test]
    fn part1() {
        // 1. print FieldElement
        println!("{}", FieldElement::new(3221225472) + FieldElement::new(10));

        // 2. construct a list of length 1023 whose first two elements are 1 and 3141592
        let mut elements = vec![FieldElement::one(), FieldElement::new(3141592)];
        while elements.len() < 1023 {
            elements
                .push(elements[elements.len() - 1].pow(2) + elements[elements.len() - 2].pow(2));
        }
        assert!(elements.len() == 1023);
        assert!(elements[0] == FieldElement::one());
        println!("{}", elements[0].pow(2) + elements[1].pow(2));
        for i in 2..elements.len() {
            assert!(elements[i] == elements[i - 1].pow(2) + elements[i - 2].pow(2));
        }
        println!("{}", elements[1022]);
        assert!(elements[1022] == FieldElement::new(2338775057));
        println!("Part 1 passed");

        // 3. interpolate the polynomial that passes through the points (0, 1), (1, 3141592) ...
        // find a subgroup of size 1024 (给定有限域的所构成的乘法群是一个阶为3*2^30的循环群，因此一定存在2^i大小的乘法子群)
        let g = FieldElement::generator().pow(3 * (1 << 20));
        let mut G = vec![];
        for i in 0..1024 {
            G.push(g.pow(i));
        }
        assert!(g.is_order(1024));
        let mut b = FieldElement::one();
        for &item in G.iter().take(1023) {
            assert!(item == b);
            b = b * g;
            assert!(b != FieldElement::one());
        }
        if b * g == FieldElement::one() {
            println!("g is a generator of the subgroup of size 1024");
        } else {
            println!("g is not a generator of the subgroup of size 1024");
        }

        // 4. evaluating the polynomial on a larger domain
        let h = FieldElement::generator().pow(3 * (1 << 30) / 8192);
        let mut H = vec![];
        for i in 0..8192 {
            H.push(h.pow(i));
        }
        let mut eval_domain = vec![];
        let w = FieldElement::generator();

        for &item in H.iter().take(8192) {
            eval_domain.push(w * item);
        }
        // let w_inv = w.inverse();
        let mut hasher = Sha256::new();
        hasher.update(H[1].val.to_string());
        let res = hasher.finalize();
        println!("{}", H[1]); // 1734477367
        println!("{}", res.to_vec().encode_hex::<String>()); // 957ebc19754464f1dc110b6f7683961c2abf380955da4124888902763806beaa
    }
}
