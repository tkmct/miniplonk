use ark_ff::Field;

use crate::circuit::Circuit;
//
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
