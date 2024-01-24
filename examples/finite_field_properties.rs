use finite_field::FieldElement;
use primes::is_prime;

fn main() {
    // closed set
    let a: FieldElement<i8> = FieldElement::new(14, 17);
    let b: FieldElement<i8> = FieldElement::new(9, 17);

    let c = a + b;
    let d = a * b;

    assert_eq!(c, FieldElement::new(6, 17));
    assert_eq!(d, FieldElement::new(7, 17));   

    // additive identity
    let a: FieldElement<i32> = FieldElement::new(5, 7);
    let zero: FieldElement<i32> = FieldElement::new(0, 7);
    let b = a + zero;
    assert_eq!(a, b); 

    // multiplicative identity
    let a: FieldElement<i16> = FieldElement::new(18, 19);
    let one: FieldElement<i16> = FieldElement::new(1, 19);
    let b = a * one;
    assert_eq!(a, b);

    // additive inverse (unsigned)
    let a: FieldElement<u32> = FieldElement::new(5, 31);
    let zero: FieldElement<u32> = FieldElement::new(0, 31);
    let a_minus = zero - a;
    assert_eq!(a_minus, FieldElement::new(26, 31));
    assert_eq!(a + a_minus, zero);

    // additive inverse (signed)
    let a: FieldElement<i32> = FieldElement::new(5, 31);
    let zero: FieldElement<i32> = FieldElement::new(0, 31);
    let a_minus = zero - a;
    assert_eq!(a_minus, FieldElement::new(26, 31));
    assert_eq!(a + a_minus, zero);

    // multiplicative inverse
    assert!(is_prime(10007));
    let a: FieldElement<u64> = FieldElement::new(324, 10007);
    let one: FieldElement<u64> = FieldElement::new(1, 10007);
    let a_inverse = one / a;
    assert_eq!(a_inverse, FieldElement::new(8926, 10007));
    assert_eq!(a * a_inverse, one);

}
