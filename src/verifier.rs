use ark_ff::FftField;

use crate::{
    circuit::Circuit,
    types::{Proof, PublicParameters},
};

pub struct Verifier<F: FftField> {
    circuit: Circuit,
    public_inputs: Vec<F>,
}

impl<F: FftField> Verifier<F> {
    pub fn new(circuit: Circuit, pp: PublicParameters, public_inputs: Vec<F>) -> Self {
        Self {
            circuit,
            public_inputs,
        }
    }

    pub fn verify(&mut self, proof: Proof) -> bool {
        todo!()
    }
}
