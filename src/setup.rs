use ark_crypto_primitives::sponge::CryptographicSponge;
use ark_ff::{FftField, PrimeField};
use ark_poly::{
    univariate::DensePolynomial, DenseUVPolynomial, EvaluationDomain, Evaluations,
    GeneralEvaluationDomain,
};
use ark_poly_commit::{kzg10::KZG10, PolynomialCommitment as PCS};

use anyhow::Result;

use crate::{types::PublicParameters, Circuit};

// compute inputs polynomial
// this can be done in setup phase
fn compute_public_input_polynomial<F>(
    circ: &Circuit,
    public_inputs: &[F],
) -> Result<DensePolynomial<F>>
where
    F: FftField + PrimeField,
{
    circ.input_config.n_pub();
    let evals = public_inputs.to_vec();
    let domain_size = circ.n_cells().checked_next_power_of_two().unwrap();

    let domain = GeneralEvaluationDomain::<F>::new(domain_size).unwrap();
    let evaluations = Evaluations::from_vec_and_domain(evals, domain);
    let poly = evaluations.interpolate();
    Ok(poly)
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
    F: FftField + PrimeField,
{
    let pi_poly = compute_public_input_polynomial(circ, public_input);
    let selector_poly = compute_selector_polynomial();

    // testing copy constraints by permutation argument

    let pp = PublicParameters {};
    pp
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::{Circuit, CircuitBuilder, InputConfig};
    use ark_bls12_381::Fq;
    use ark_poly::{
        univariate::DensePolynomial, DenseUVPolynomial, EvaluationDomain, GeneralEvaluationDomain,
        Polynomial,
    };

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
    fn test_compute_public_input_polynomial() {
        // Public input polynomial should be defined over the same domain with trace polynomial.
        // evaluation point in public input is
        // 𝜔^-i where i∈{1,..,n_pub}
        // 𝜔 is n-th root of unity where n is total number of cells in a circuit.

        let circ = simple_circ();
        let public_input = vec![Fq::from(3), Fq::from(5)];

        let n_cells = circ.n_cells();
        let n_pub = circ.input_config.n_pub();
        let domain_size = circ.n_cells().checked_next_power_of_two().unwrap();
        let domain = GeneralEvaluationDomain::<Fq>::new(domain_size).unwrap();

        let poly = compute_public_input_polynomial(&circ, &public_input).unwrap();

        for (p, e) in public_input
            .iter()
            .rev()
            .zip(domain.elements().skip(n_cells - n_pub))
        {
            let val = poly.evaluate(&e);
            assert_eq!(*p, val);
        }
    }
}
