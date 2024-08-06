#![allow(non_snake_case, dead_code)]
use crate::{remove_trailing_elements, FieldElement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    pub coeffs: Vec<FieldElement>,
}

impl Polynomial {
    fn X() -> Self {
        Polynomial::new(vec![FieldElement::zero(), FieldElement::one()])
    }

    fn new(mut coeffs: Vec<FieldElement>) -> Self {
        remove_trailing_elements::<FieldElement>(&mut coeffs, FieldElement::zero());
        Polynomial { coeffs }
    }

    fn add(&self, other: &Self) -> Self {
        let mut new_coeffs = vec![];
        let mut i = 0;
        while i < self.coeffs.len() || i < other.coeffs.len() {
            let mut sum = FieldElement::zero();
            if i < self.coeffs.len() {
                sum = sum + self.coeffs[i];
            }
            if i < other.coeffs.len() {
                sum = sum + other.coeffs[i];
            }
            new_coeffs.push(sum);
            i += 1;
        }
        Polynomial::new(new_coeffs)
    }

    fn sub(&self, other: &Self) -> Self {
        let mut new_coeffs = vec![];
        let mut i = 0;
        while i < self.coeffs.len() || i < other.coeffs.len() {
            let mut diff = FieldElement::zero();
            if i < self.coeffs.len() {
                diff = diff + self.coeffs[i];
            }
            if i < other.coeffs.len() {
                diff = diff - other.coeffs[i];
            }
            new_coeffs.push(diff);
            i += 1;
        }
        Polynomial::new(new_coeffs)
    }

    fn mul(&self, other: &Self) -> Self {
        let mut new_coeffs = vec![FieldElement::zero(); self.coeffs.len() + other.coeffs.len() - 1];
        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                new_coeffs[i + j] = new_coeffs[i + j] + self.coeffs[i] * other.coeffs[j];
            }
        }
        Polynomial::new(new_coeffs)
    }

    fn neg(&self) -> Self {
        let mut new_coeffs = vec![];
        for coeff in &self.coeffs {
            new_coeffs.push(coeff.negate());
        }
        Polynomial::new(new_coeffs)
    }

    fn compose(&self, other: &Self) -> Self {
        let mut res = Polynomial::from(FieldElement::zero());
        for &coeff in self.coeffs.iter().rev() {
            res = res.mul(other) + Polynomial::from(coeff);
        }
        res
    }
}

impl std::ops::Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut new_coeffs = vec![];
        let mut i = 0;
        while i < self.coeffs.len() || i < other.coeffs.len() {
            let mut sum = FieldElement::zero();
            if i < self.coeffs.len() {
                sum = sum + self.coeffs[i];
            }
            if i < other.coeffs.len() {
                sum = sum + other.coeffs[i];
            }
            new_coeffs.push(sum);
            i += 1;
        }
        Polynomial::new(new_coeffs)
    }
}

impl std::ops::Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut new_coeffs = vec![];
        let mut i = 0;
        while i < self.coeffs.len() || i < other.coeffs.len() {
            let mut diff = FieldElement::zero();
            if i < self.coeffs.len() {
                diff = diff + self.coeffs[i];
            }
            if i < other.coeffs.len() {
                diff = diff - other.coeffs[i];
            }
            new_coeffs.push(diff);
            i += 1;
        }
        Polynomial::new(new_coeffs)
    }
}

impl std::ops::Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut new_coeffs = vec![FieldElement::zero(); self.coeffs.len() + other.coeffs.len() - 1];
        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                new_coeffs[i + j] = new_coeffs[i + j] + self.coeffs[i] * other.coeffs[j];
            }
        }
        Polynomial::new(new_coeffs)
    }
}

impl std::ops::Neg for Polynomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut new_coeffs = vec![];
        for coeff in &self.coeffs {
            new_coeffs.push(coeff.negate());
        }
        Polynomial::new(new_coeffs)
    }
}

impl From<FieldElement> for Polynomial {
    fn from(f: FieldElement) -> Self {
        Polynomial::new(vec![f])
    }
}

impl From<u64> for Polynomial {
    fn from(i: u64) -> Self {
        Polynomial::from(FieldElement::new(i))
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_poly() {
        let p1 = Polynomial::new(vec![FieldElement::new(1), FieldElement::new(2)]);
        let p2 = Polynomial::new(vec![FieldElement::new(3), FieldElement::new(4)]);
        let p3 = Polynomial::new(vec![FieldElement::new(4), FieldElement::new(6)]);
        assert_eq!(p1 + p2, p3);

        let p1 = Polynomial::new(vec![FieldElement::new(1), FieldElement::new(2)]);
        let p2 = Polynomial::new(vec![FieldElement::new(3), FieldElement::new(4)]);
        let p3 = Polynomial::new(vec![
            FieldElement::new(3),
            FieldElement::new(10),
            FieldElement::new(8),
        ]);
        assert_eq!(p1 * p2, p3);
    }

    #[test]
    fn test_compose() {
        let p1 = Polynomial::new(vec![
            FieldElement::new(0),
            FieldElement::new(1),
            FieldElement::new(1),
        ]);
        let p2 = Polynomial::new(vec![FieldElement::new(1), FieldElement::new(1)]);
        let p3 = Polynomial::new(vec![
            FieldElement::new(2),
            FieldElement::new(3),
            FieldElement::new(1),
        ]);
        println!("{:?}", p1.compose(&p2));
        assert_eq!(p1.compose(&p2), p3);
    }
}
