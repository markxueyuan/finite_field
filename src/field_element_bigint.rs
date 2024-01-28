use std::ops::{Add, Sub, Mul, Div};
use crypto_bigint::modular::runtime_mod::{DynResidueParams, DynResidue};
use crypto_bigint::{Checked, NonZero, Uint};



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FieldElement<const LIMBS: usize> {
    n: Uint<LIMBS>,
    order: Uint<LIMBS>,
}

impl<const LIMBS: usize> FieldElement<LIMBS>
{
    pub fn new(n: Uint<LIMBS>, order: Uint<LIMBS>) -> Self {
        let modulus = NonZero::new(order).unwrap();
        let n = n % modulus;
        FieldElement {
            n,
            order
        }
    }    

    pub fn get_num(&self) -> Uint<LIMBS> {
        self.n    
    }    
}

impl<const LIMBS: usize> Add for FieldElement<LIMBS> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        assert_eq!(self.order, rhs.order);
        let residue_params = DynResidueParams::new(&self.order);

        let residue1 = DynResidue::new(&self.n, residue_params);
        let residue2 = DynResidue::new(&rhs.n, residue_params);
        let n = (residue1 + residue2).retrieve();
        FieldElement {
            n,
            order: self.order    
        } 
    }    
}

impl<const LIMBS: usize> Sub for FieldElement<LIMBS> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        assert_eq!(self.order, rhs.order);
        let residue_params = DynResidueParams::new(&self.order);

        let residue1 = DynResidue::new(&self.n, residue_params);
        let residue2 = DynResidue::new(&rhs.n, residue_params);
        let n = (residue1 - residue2).retrieve();
        FieldElement {
            n,
            order: self.order    
        } 
    }    
}


impl<const LIMBS: usize> Mul for FieldElement<LIMBS> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        assert_eq!(self.order, rhs.order);
        let residue_params = DynResidueParams::new(&self.order);

        let residue1 = DynResidue::new(&self.n, residue_params);
        let residue2 = DynResidue::new(&rhs.n, residue_params);
        let n = (residue1 * residue2).retrieve();
        FieldElement {
            n,
            order: self.order    
        } 
    }    
}

impl<const LIMBS: usize> FieldElement<LIMBS> {

    pub fn pow(self, exp: Uint<LIMBS>) -> Self {
        /* let one = Uint::from(1u8);   
        let nonzero = NonZero::new(self.order.sub_mod(&one, &self.order)).unwrap();
        let exp = exp % nonzero; */
        let residue_params = DynResidueParams::new(&self.order);

        let residue = DynResidue::new(&self.n, residue_params);
        let n = (residue.pow(&exp)).retrieve();
        FieldElement {
            n,
            order: self.order    
        } 
    }    
}



impl<const LIMBS: usize> Div for FieldElement<LIMBS> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert_eq!(self.order, rhs.order);
        let two = Uint::from(2u8);
        let residue_params = DynResidueParams::new(&self.order);

        let residue1 = DynResidue::new(&self.n, residue_params);
        let residue2 = DynResidue::new(&rhs.n, residue_params);
        // a/b = a * b^(-1) = a * b^(p-2)
        let p_sub_2 = (Checked::new(self.order) - Checked::new(two)).0.unwrap();
        let n = (residue1 * residue2.pow(&p_sub_2)).retrieve();
        FieldElement {
            n,
            order: self.order    
        } 
    }    
}


#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::{U256, U512};

    #[test]
    fn new_works() {
        let num = U256::from(9u8);
        let modulus = U256::from(7u8);
        let element = FieldElement::new(num, modulus);
        assert!(element.n < element.order);
    }

    #[test]    
    fn add_works() {
        let modulus = U256::from(7u8);    
        
        let num1 = U256::from(5u8);
        let num2 = U256::from(6u8);
        let num3 = U256::from(4u8);


        let elm1 = FieldElement::new(num1, modulus);
        let elm2 = FieldElement::new(num2, modulus);       
        let elm3 = FieldElement::new(num3, modulus);
        assert_eq!(elm1 + elm2, elm3);
    }    

    #[test]
    fn sub_works() {
        let modulus = U256::from(7u8);    
        
        let num1 = U256::from(5u8);
        let num2 = U256::from(6u8);
        let num3 = U256::from(1u8);


        let elm1 = FieldElement::new(num1, modulus);
        let elm2 = FieldElement::new(num2, modulus);       
        let elm3 = FieldElement::new(num3, modulus);
        assert_eq!(elm1 - elm2, elm2);
        assert_eq!(elm2 - elm1, elm3);
    }    

    #[test]
    fn mul_works() {
        let modulus = U256::from(7u8);    
        
        let num1 = U256::from(5u8);
        let num2 = U256::from(6u8);
        let num3 = U256::from(2u8);


        let elm1 = FieldElement::new(num1, modulus);
        let elm2 = FieldElement::new(num2, modulus);       
        let elm3 = FieldElement::new(num3, modulus);
        assert_eq!(elm1 * elm2, elm3);
    }        

    #[test]
    fn pow_works() {
        let modulus = U256::from(7u8);    
        
        let num1 = U256::from(3u8);
        let exp = U256::from(3u8);
        let num2 = U256::from(6u8);


        let elm1 = FieldElement::new(num1, modulus);
        let elm2 = FieldElement::new(num2, modulus);
        assert_eq!(elm1.pow(exp), elm2);

        let num1 = U256::from(2u8);
        let exp = U256::from(8u8);
        let num2 = U256::from(4u8);


        let elm1 = FieldElement::new(num1, modulus);
        let elm2 = FieldElement::new(num2, modulus);
        assert_eq!(elm1.pow(exp), elm2);
    }    

    #[test]
    fn div_works() {
        let modulus = U512::from(7u8);    
        
        let num1 = U512::from(5u8);
        let num2 = U512::from(6u8);
        let num3 = U512::from(2u8);


        let elm1 = FieldElement::new(num1, modulus);
        let elm2 = FieldElement::new(num2, modulus);       
        let elm3 = FieldElement::new(num3, modulus);
        assert_eq!(elm3 / elm2, elm1);   
        assert_eq!(elm3 / elm1, elm2);
    }    
}
