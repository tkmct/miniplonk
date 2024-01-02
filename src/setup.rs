use ark_bls12_381::Bls12_381;
use ark_ec::pairing::Pairing;
use ark_ff::{FftField, PrimeField};
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, Evaluations, GeneralEvaluationDomain,
};
use ark_poly_commit::kzg10::KZG10;
use ark_std::rand::RngCore;

use anyhow::Result;

use crate::{
    common::{compute_public_input_polynomial, compute_selector_polynomial},
    types::{PublicParameters, UniPoly381},
    Circuit,
};

/// setup public parameters
///
/// * `circ` - Circuit to prove.
/// * `public_input` - Public input to a circuit.
/// * `rng` - random number generator used to setup KZG
/// * `degree` - Maximum degree of KZG.
pub fn setup<F, R>(
    circ: &Circuit,
    public_input: &[F],
    rng: &mut R,
    degree: usize,
) -> Result<PublicParameters>
where
    F: FftField + PrimeField,
    R: RngCore,
{
    let v_poly = compute_public_input_polynomial(circ, public_input);
    let s_poly = compute_selector_polynomial::<F>(circ);

    // testing copy constraints by permutation argument

    // Setup poly commit

    let params = KZG10::<Bls12_381, UniPoly381>::setup(degree, false, rng)?;
    let pp = PublicParameters { kzg_params: params };

    Ok(pp)
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
        let pad = vec![Fq::from(0); n_cells - n_pub];

        for (v, d) in pad
            .iter()
            .chain(public_input.iter().rev())
            .zip(domain.elements())
        {
            let val = poly.evaluate(&d);
            assert_eq!(*v, val);
        }
    }

    #[test]
    fn test_compute_selector_polynomial() {
        let circ = simple_circ();

        let poly = compute_selector_polynomial(&circ).unwrap();
        let domain_size = circ.n_cells().checked_next_power_of_two().unwrap();
        let domain = GeneralEvaluationDomain::<Fq>::new(domain_size).unwrap();

        let expected = [1, 0, 1].iter().map(|i| Fq::from(*i)).collect::<Vec<_>>();

        for (d, v) in domain.elements().step_by(3).zip(expected) {
            let val = poly.evaluate(&d);
            assert_eq!(v, val);
        }
    }
}
