use std::{fmt::Debug, ops::{Add, Div, Mul, Rem, Sub}};
use num::{One, Zero, Num, Bounded};
use std::ops::Shr;
use mod_exp::mod_exp;

/// This crate implements the finite fields for generic types.
/// The mathematic definitions of finite fields are discussed in Chapter One 
/// of Programming Bitcoin by Jimmy Song. 


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldElement<T> 
{    
    n: T,
    order: T
}

impl<T> FieldElement<T>
    where T: PartialOrd + Zero,
{
    pub fn new(n: T, order: T) -> Self { 
        assert!(n >= Zero::zero() && n < order);
        FieldElement {
            n,
            order    
        }   
    }        
}

impl<T> Add for FieldElement<T>
    where T: PartialOrd + Debug + Add<Output = T> + Rem<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order, rhs.order);
        let n = self.n + rhs.n;
        let p = self.order;
        let n = n % p;
        FieldElement {
            n,
            order: p     
        }
    }    
}

impl<T> Sub for FieldElement<T>
    where T: PartialOrd + Debug + Sub<Output = T> + Rem<Output = T> + Zero + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.order, rhs.order);
        let p = self.order;
        let n: T;
        if self.n < rhs.n {
            let t = rhs.n - self.n;    
            let t = t % p;
            n = p - t;
        } else {
            let t = self.n - rhs.n;
            let t = t % p;
            n = t;    
        }
        FieldElement {
            n,
            order: p    
        }
    }        
} 

impl<T> Mul for FieldElement<T>
    where T: Mul<Output = T> + Rem<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let n =  self.n * rhs.n;
        let p = self.order;
        let n = n % p;
        FieldElement {
            n,
            order: p    
        }
    }    
}    

impl<T> Div for FieldElement<T>
    where T: Num + PartialOrd + Shr<T, Output = T> + Copy + Bounded,
          T: Mul<Output = T> + Rem<Output = T> + Zero,
{
    type Output = Self;    
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.n == Zero::zero() {
            panic!("Zero is not valid denominator.")

        }    
        let a = self.n;
        let b = rhs.n;
        let p = self.order;
        let fermat = mod_exp(b, p - One::one() - One::one(), p);
        let n = (a * fermat) % p;

        FieldElement {
            n,
            order: p
        }
    }    
}

impl<T> FieldElement<T> 
    where T: One + Sub<Output = T> + Rem<Output = T> + Add<Output = T> + Copy,
          T: Num + PartialOrd + Shr<T, Output = T> + Bounded,
{
    pub fn pow(self, exp: T) -> Self {
        let p = self.order - One::one();
        let n = (exp % p + p) % p;
        let n = mod_exp(self.n, n, self.order);

        FieldElement {
            n,
            order: self.order
        }        
    }    
}

impl<T> FieldElement<T> {
    pub fn one(self) -> FieldElement<T> 
        where T: One,    
    {
        FieldElement { n: One::one(), order: self.order }    
    }    

    pub fn zero(self) -> FieldElement<T> 
        where T: Zero
    {
        FieldElement { n: Zero::zero(), order: self.order }    
    }    
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let a: FieldElement<i8> = FieldElement::new(3, 5);
        let b: FieldElement<i8> = FieldElement {
            n: 3,
            order: 5,    
        };
        assert_eq!(a, b);
    }    

   #[test]
    fn add_i32_works() {
        let a: FieldElement<i32> = FieldElement::new(7, 19);   
        let b: FieldElement<i32> = FieldElement::new(18, 19);
        let c = a + b;
        println!("{:?}", c);
        assert_eq!(c, FieldElement::new(6, 19));
    }    

    #[test]
    fn add_i8_works() {
        let a: FieldElement<i8> = FieldElement::new(9, 11);
        let b: FieldElement<i8> = FieldElement::new(3, 11);
        let c = a + b;
        println!("{:?}", c);
        assert_eq!(c, FieldElement::new(1, 11));
    }

    #[test]
    fn sub_i64_works() {
        let a: FieldElement<i64> = FieldElement::new(5, 11);
        let b: FieldElement<i64> = FieldElement::new(9, 11);
        let c = a - b;
        let d = b - a;
        println!("{:?}", c);
        assert_eq!(c, FieldElement::new(7, 11));
        assert_eq!(c + d, FieldElement::new(0, 11));
    }    

    #[test]
    fn sub_u64_works() {
        let a: FieldElement<u64> = FieldElement::new(5, 11);
        let b: FieldElement<u64> = FieldElement::new(9, 11);
        let c = a - b;
        let d = b - a;
        println!("{:?}", c);
        assert_eq!(c, FieldElement::new(7, 11));
        assert_eq!(c + d, FieldElement::new(0, 11));
    }    

    #[test]
    fn mul_i16_works() {
        let a: FieldElement<i16> = FieldElement::new(6, 7);
        let b: FieldElement<i16> = FieldElement::new(3, 7);
        assert_eq!(a * b, FieldElement::new(4, 7));            
    }

    #[test]
    fn div_u16_works() {
        let x: FieldElement<u16> = FieldElement::new(4, 7);
        let y: FieldElement<u16> = FieldElement::new(6, 7);
        assert_eq!(x / y, FieldElement::new(3, 7));
    }

    #[test]
    fn pow_works() {
        let x: FieldElement<i32> = FieldElement::new(4, 7);
        let one: FieldElement<i32> = FieldElement::new(1, 7);
        assert_eq!(x.pow(-17), one / x.pow(17));
    }    

    #[test]
    fn binary_works() {
        let one: FieldElement<i8> = FieldElement::new(1, 2);
        let zero: FieldElement<i8> = FieldElement::new(0, 2);
        assert_eq!(zero - one, FieldElement::new(1, 2));
    }    

    #[test]
    fn identity_works() {
        let a: FieldElement<i32> = FieldElement::new(5, 31);
        let one = a.one();
        let zero = a.zero();

        assert_eq!(a * one, a);
        assert_eq!(a + zero, a);            
    }    

}
