use ark_ff::{FftField, PrimeField};
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, Evaluations, GeneralEvaluationDomain,
};

use anyhow::Result;

use crate::{circuit::Op, Circuit};

///
/// compute inputs polynomial
/// this can be done in setup phase
pub(crate) fn compute_public_input_polynomial<F>(
    circ: &Circuit,
    public_input: &[F],
) -> Result<DensePolynomial<F>>
where
    F: FftField,
{
    let n_cells = circ.n_cells();
    let evals = public_input.to_vec();
    let domain_size = circ.n_cells().checked_next_power_of_two().unwrap();

    let mut pad = vec![F::zero(); n_cells - evals.len()];
    pad.append(&mut evals.iter().rev().copied().collect::<Vec<_>>());

    let domain = GeneralEvaluationDomain::<F>::new(domain_size).unwrap();
    let evaluations = Evaluations::from_vec_and_domain(pad, domain);
    let poly = evaluations.interpolate();

    Ok(poly)
}

/// compute selector polynomial independent of inputs
/// this can be done in setup phase
pub(crate) fn compute_selector_polynomial<F>(circ: &Circuit) -> Result<DensePolynomial<F>>
where
    F: FftField,
{
    // compute selector polynomial from circuit
    let domain_size = circ.n_cells().checked_next_power_of_two().unwrap();

    let selectors = circ
        .selectors
        .iter()
        .map(|op| match op {
            Op::Add => F::ONE,
            Op::Mul => F::ZERO,
        })
        .collect::<Vec<_>>();

    let mut evals = vec![selectors[0]];
    selectors.iter().skip(1).for_each(|v| {
        evals.push(F::ZERO);
        evals.push(F::ZERO);
        evals.push(*v);
    });

    let domain = GeneralEvaluationDomain::<F>::new(domain_size).unwrap();
    let evaluations = Evaluations::from_vec_and_domain(evals, domain);
    let poly = evaluations.interpolate();
    Ok(poly)
}

// polynomial which implements rotation like followings
// W(𝜔-2 , 𝜔1 , 𝜔3) = (𝜔1 , 𝜔3 , 𝜔-2 ) , W(𝜔-1 , 𝜔0) = (𝜔0 , 𝜔-1), ,,,
fn compute_wire_rotation_polynomial() {
    todo!()
}
