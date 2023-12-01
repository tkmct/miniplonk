use anyhow::{anyhow, Result};
use ark_ff::FftField;
use ark_poly::{
    univariate::DensePolynomial, DenseUVPolynomial, EvaluationDomain, GeneralEvaluationDomain,
    Radix2EvaluationDomain,
};
use std::collections::VecDeque;

use crate::circuit::{Circuit, Op};

// TODO: change this
pub type Proof = u64;

pub struct Prover<F: FftField> {
    circuit: Circuit,
    public_inputs: Vec<F>,
    private_inputs: Vec<F>,

    /// This field stores complete witness data.
    computation_trace: Option<Vec<F>>,
}

impl<F: FftField> Prover<F> {
    /// Create new prover instance
    pub fn new(circuit: Circuit, public_inputs: Vec<F>, private_inputs: Vec<F>) -> Self {
        Self {
            circuit,
            public_inputs,
            private_inputs,
            computation_trace: None,
        }
    }

    /// Calculate all intermediate witness values in a circuit gate by gate.
    pub fn calculate_witness(&mut self) -> Result<()> {
        // assign input wirings to the cells
        let n_inputs = self.circuit.n_inputs();
        let n_cells = self.circuit.n_cells();
        let n_rows = self.circuit.n_rows();

        let mut trace: Vec<Option<F>> = vec![None; n_cells];
        let mut eval_queue = VecDeque::<usize>::new();

        for i in 0..n_inputs {
            let value = self
                .get_input_val(i)
                .expect("Value within 0..total_input should not panic");

            // assign input cell and its copy constrained cells
            let id = n_cells - (i + 1);
            trace[id] = Some(value);

            self.circuit
                .get_copy_constraints(id)
                .unwrap()
                .iter()
                .for_each(|cell_id| {
                    trace[*cell_id] = Some(value);

                    let row = cell_id / 3;
                    if row < n_rows {
                        // This is actually a gate constraint
                        let lhs = trace[row * 3];
                        let rhs = trace[row * 3 + 1];
                        let out = trace[row * 3 + 2];
                        if lhs.is_some() && rhs.is_some() && out.is_none() {
                            eval_queue.push_back(row * 3 + 2);
                        }
                    }
                });
        }

        // loop queue until it's all calculated
        while let Some(id) = eval_queue.pop_front() {
            let lhs = trace[id - 2].unwrap();
            let rhs = trace[id - 1].unwrap();
            let op = self.circuit.get_selector(id / 3).unwrap();
            let value = match op {
                Op::Add => lhs + rhs,
                Op::Mul => lhs * rhs,
            };

            print!(
                "Id: {}, Assign value {}({}, {}) = {}",
                id,
                if op == Op::Add { "ADD" } else { "MUL" },
                lhs,
                rhs,
                value
            );

            trace[id] = Some(value);
            if id == self.circuit.output_id() {
                continue;
            }

            self.circuit
                .get_copy_constraints(id)
                .unwrap()
                .iter()
                .for_each(|cell_id| {
                    trace[*cell_id] = Some(value);

                    let row = cell_id / 3;
                    if row < n_rows {
                        // This is actually a gate constraint
                        let lhs = trace[row * 3];
                        let rhs = trace[row * 3 + 1];
                        let out = trace[row * 3 + 2];
                        if lhs.is_some() && rhs.is_some() && out.is_none() {
                            eval_queue.push_back(row * 3 + 2);
                        }
                    }
                })
        }

        debug_assert!(trace.iter().all(|o| o.is_some()), "");

        let trace = trace
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or(anyhow!("Not all the cells are filled"))?;
        self.computation_trace = Some(trace);

        Ok(())
    }

    // Compute polynomial that represents whole computation trace.
    pub fn compute_trace_polynomial(&self) -> Result<DensePolynomial<F>> {
        // Evaluation domain should better be radix-2 for efficient FFT.
        // domain should be 2^n
        let size = self
            .circuit
            .n_cells()
            .checked_next_power_of_two()
            .ok_or(anyhow!("Circuit size is too large."))?;

        let domain = GeneralEvaluationDomain::<F>::new(size)
            .ok_or(anyhow!("Domain cannot be constructed from circuit size"))?;

        let trace = self
            .computation_trace
            .clone()
            .ok_or(anyhow!("Computation should be complete."))?;

        Ok(DensePolynomial::from_coefficients_vec(domain.ifft(&trace)))
    }

    // compute inputs polynomial
    // this can be done in setup phase
    fn compute_inputs_polynomial(&self) {
        todo!()
    }

    // compute selector polynomial independent of inputs
    // this can be done in setup phase
    fn compute_selector_polynomial(&self) {
        todo!()
    }

    // polynomial which implements rotation like followings
    // W(𝜔-2 , 𝜔1 , 𝜔3) = (𝜔1 , 𝜔3 , 𝜔-2 ) , W(𝜔-1 , 𝜔0) = (𝜔0 , 𝜔-1), ,,,
    fn compute_wire_rotation_polynomial(&self) {
        todo!()
    }

    fn prove(self) -> Proof {
        // suppose we have polynomial commitment scheme available like KZG.
        // generate computation trace
        // setup and cmopute inputs polynomial and selector polynomial => prover parameter
        // commits to those polynomials. => verifier parameter
        //
        // generate witness
        // calculate trace polynomial and commits to it.
        // calculate wire rotation polynomial.
        //
        // prove following things using polynomial checks
        // 1. gates
        // 2. inputs
        // 3. wires
        // 4. output
        todo!()
    }

    fn get_input_val(&self, id: usize) -> Option<F> {
        if id < self.public_inputs.len() {
            self.public_inputs.get(id).copied()
        } else {
            self.private_inputs
                .get(id - self.public_inputs.len())
                .copied()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::{Circuit, CircuitBuilder, InputConfig};
    use ark_bls12_381::Fq;
    use ark_poly::Polynomial;

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
    fn test_generate_witness() {
        // Input Cells
        // | pub_0 | pub_1 | priv_0 |
        //
        // Wire Cells
        // | lhs   | rhs    | out   | s |
        // |-------|--------|-------|---|
        // | pub_0 | priv_0 | out_0 | 0 |
        // | out_0 | pub_1  | out_1 | 1 |
        // | out_1 | priv_0 | out   | 0 |
        //
        // Witness should assigned as following
        //
        // Input Cells
        // | 3 | 5 | 7 |
        //
        // Wire Cells
        // | lhs  | rhs  | out  | s |
        // |------|------|------|---|
        // |  3   |  7   |  10  | 0 |
        // | 10   |  5   |  50  | 1 |
        // | 50   |  7   |  57  | 0 |

        let circ = simple_circ();
        let public_inputs = vec![Fq::from(3), Fq::from(5)];
        let private_inputs = vec![Fq::from(7)];
        let mut prover = Prover::<Fq>::new(circ, public_inputs, private_inputs);

        let result = prover.calculate_witness();
        let expected = [3, 7, 10, 10, 5, 50, 50, 7, 57, 7, 5, 3]
            .iter()
            .map(|i| Fq::from(*i))
            .collect::<Vec<_>>();

        assert!(result.is_ok(), "Witness should be correctly calculated");
        assert!(
            prover.computation_trace.unwrap().eq(&expected),
            "Witness should be calculated correctly."
        );
    }

    #[test]
    fn test_trace_polynomial() {
        let circ = simple_circ();

        let size = circ.n_cells().checked_next_power_of_two().unwrap();
        let public_inputs = vec![Fq::from(3), Fq::from(5)];
        let private_inputs = vec![Fq::from(7)];
        let mut prover = Prover::<Fq>::new(circ, public_inputs, private_inputs);

        let _ = prover.calculate_witness();
        let expected = [3, 7, 10, 10, 5, 50, 50, 7, 57, 7, 5, 3]
            .iter()
            .map(|i| Fq::from(*i))
            .collect::<Vec<_>>();

        let result = prover.compute_trace_polynomial();
        assert!(
            result.is_ok(),
            "Trace polynomial should be correctly calculated."
        );

        let poly = result.unwrap();
        let domain = GeneralEvaluationDomain::<Fq>::new(size).unwrap();
        for (w, e) in expected.iter().zip(domain.elements().take(expected.len())) {
            let val = poly.evaluate(&e);
            assert_eq!(*w, val);
        }
    }
}
