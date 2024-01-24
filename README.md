This crate implements the finite fields for generic types.

The mathematic definitions of finite fields are discussed in Chapter One of *Programming Bitcoin* by Jimmy Song. 

# Example

```rust
use finite_field::FieldElement;

let a: FieldElement<u64> = FieldElement::new(5, 7);
let zero = a.zero();

let b = zero - a;
assert_eq!(b, FieldElement::new(2, 7));

```

Refer to the "examples" folder for more demonstration.