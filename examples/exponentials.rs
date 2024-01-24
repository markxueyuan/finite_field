use finite_field::FieldElement;

fn main () {
    // negative exponents for signed bases.
    let a: FieldElement<i16> = FieldElement::new(15, 31);
    let one: FieldElement<i16> = FieldElement::new(1, 31);
    let exp = a.pow(-3);
    assert_eq!(one / exp, a * a * a);
    assert_eq!(one / exp, a.pow(3));

    // positive exponents for unsigned bases.
    let a: FieldElement<u16> = FieldElement::new(15, 31);
    let exp = a.pow(5);
    assert_eq!(exp, a * a * a * a * a);

    // negative exponents for unsigned bases;
    // Since it requires that the exponents and bases have the same type,
    // negative exponents for unsigned bases are not implemented
    // are not implemented, a workaround is to compute the reverse:
    let one: FieldElement<u16> = FieldElement::new(1, 31);
    // exp = a^(-5)
    let exp = one / a.pow(5);
    assert_eq!(exp, one / (a * a * a * a * a)) 

    
}