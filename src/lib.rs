mod circuit;
mod prover;

pub use circuit::{Circuit, CircuitBuilder};
pub use prover::Prover;

/*
integration test
fn test_prove_and_verify() {
    // prove following knowledge of witness
    // out = (x1 + x2)(x2 + w)
    // represent circuit as a computation trace

    // represent computation with polynomials

    // commit to the polynomials

    // proofs for witness satisfy certain equations
}
*/
