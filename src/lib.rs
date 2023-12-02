mod circuit;
mod prover;
mod setup;
mod types;
mod verifier;

pub use circuit::{Circuit, CircuitBuilder};
pub use prover::Prover;
pub use setup::setup;
pub use types::{Proof, PublicParameters};
pub use verifier::Verifier;

#[cfg(test)]
mod tests {
    use crate::{circuit::*, prover::*, setup::*, verifier::*};

    use ark_bls12_381::Fq as F;

    // build circuit to calculate
    // out = (pub_0 + priv_0) * pub_1 + priv_0
    fn simple_circ() -> Circuit {
        let mut builder = CircuitBuilder::new(InputConfig::new(2, 1));
        let (pb_refs, prv_refs) = builder.get_input_refs();
        let out_0 = builder.add_addition(pb_refs[0], prv_refs[0]).unwrap();
        let out_1 = builder.add_multiplication(out_0, pb_refs[1]).unwrap();
        let _ = builder.add_addition(out_1, prv_refs[0]).unwrap();

        builder.build().unwrap()
    }

    #[test]
    fn test_prove_and_verify() {
        // Setup public parameters
        let circ = simple_circ();
        let public_inputs = vec![F::from(3), F::from(5)];

        // setup polynomials
        let pp = setup(&circ, &public_inputs);
        let proof;

        {
            let private_inputs = vec![F::from(7)];
            let mut prover =
                Prover::<F>::new(circ.clone(), pp, public_inputs.clone(), private_inputs);
            let result = prover.calculate_witness();
            assert!(result.is_ok());
            // TODO: is output public?
            // commit to the polynomials
            proof = prover.prove();
        }

        {
            let mut verifier = Verifier::<F>::new(circ, pp, public_inputs);
            let result = verifier.verify(proof);
            assert!(result);
        }
    }
}
