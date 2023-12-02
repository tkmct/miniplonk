use ark_ff::FftField;

use crate::{types::PublicParameters, Circuit};

// compute inputs polynomial
// this can be done in setup phase
fn compute_inputs_polynomial() {
    todo!()
}

// compute selector polynomial independent of inputs
// this can be done in setup phase
fn compute_selector_polynomial() {
    todo!()
}

// polynomial which implements rotation like followings
// W(𝜔-2 , 𝜔1 , 𝜔3) = (𝜔1 , 𝜔3 , 𝜔-2 ) , W(𝜔-1 , 𝜔0) = (𝜔0 , 𝜔-1), ,,,
fn compute_wire_rotation_polynomial() {
    todo!()
}

/// setup public parameters
pub fn setup<F>(circ: &Circuit, public_input: &[F]) -> PublicParameters
where
    F: FftField,
{
    // do calculate polynomials and public parameters
    // depending on polynomial commitment scheme

    let pp = PublicParameters {};
    pp
}
