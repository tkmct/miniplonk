use ark_ff::Field;

use crate::circuit::Circuit;

// TODO: change this
pub type Proof = u64;

pub struct Prover<F: Field> {
    circuit: Circuit,
    public_inputs: Vec<F>,
    private_inputs: Vec<F>,
}

impl<F: Field> Prover<F> {
    /// Create new prover instance
    pub fn new(circuit: Circuit, public_inputs: Vec<F>, private_inputs: Vec<F>) -> Self {
        Self {
            circuit,
            public_inputs,
            private_inputs,
        }
    }

    /// Calculate all intermediate witness values in a circuit gate by gate.
    fn calculate_witness(&mut self) -> Vec<F> {
        todo!()
    }

    // this can only be done by prover
    fn compute_trace_polynomial(&self) {
        todo!()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::{Circuit, CircuitBuilder, InputConfig};
    use ark_bls12_381::Fq;

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
        let public_inputs = vec![];
        let private_inputs = vec![];
        let prover = Prover::<Fq>::new(circ, public_inputs, private_inputs);
    }
}
