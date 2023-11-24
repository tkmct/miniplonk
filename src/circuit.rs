use std::collections::HashSet;

/*
- Define private/public inputs with input configuration.
- Represent wire with column and row
          column
       0  1  2 selector
row 0
    1
    2
    3
    4
- To add a single gate, you add one row to the computation trace table
  by turning your selector column 0 or 1. 0 for addition and 1 for multiplication.
- When cells are handled by prover, table will be flattened to a single vector.
*/

type Row = usize;
type Column = usize;

// In what position of the polynomial the cell exists.
// In order to represent both intermediate cells and input cells,
// For input cells, id: total cell - input number
// For intermediate cells, id:
type Id = usize;

#[derive(Clone, Copy)]
pub enum Op {
    Add = 0,
    Mul = 1,
}

pub struct InputConfig {
    n_public_input: usize,
    n_private_input: usize,
}

/// Circuit struct
pub struct Circuit {
    input_config: InputConfig,
    selectors: Vec<Op>,
    wirings: Vec<Vec<Id>>,
    total_cells: usize,
}

pub struct CircuitBuilder {
    current_row: usize,
    ops: Vec<Op>,
    /// Store which two cells are equal
    wiring_pairs: Vec<(Id, Id)>,
    input_config: InputConfig,
}

impl CircuitBuilder {
    /// Create computation builder instance
    pub fn new(input_config: InputConfig) -> Self {
        Self {
            current_row: 0,
            ops: vec![],
            wiring_pairs: vec![],
            input_config,
        }
    }

    /// Add new constraint to a circuit.
    pub fn add_addition(&mut self) -> (Id, Id, Id) {
        self.ops.push(Op::Add);
        let pos = self.current_row * 3;
        (pos, pos + 1, pos + 2)
    }

    pub fn add_multiplication(&mut self) -> (Id, Id, Id) {
        self.ops.push(Op::Mul);
        let pos = self.current_row * 3;
        (pos, pos + 1, pos + 2)
    }

    /// Add wire constraint to a circuit.
    pub fn add_wire_constraint(&mut self, x: Id, y: Id) {
        self.wiring_pairs.push((x, y))
    }

    pub fn get_public_input_ids(&self) -> Vec<Id> {
        vec![]
    }

    pub fn get_private_input_ids(&self) -> Vec<Id> {
        vec![]
    }

    pub fn build(self) -> Circuit {
        let n_input = self.input_config.n_public_input + self.input_config.n_private_input;
        let total_cells = n_input + self.current_row * 3;

        // calculate every wirings
        let mut wirings = Vec::with_capacity(n_input);
        for input_number in 1..=n_input {
            // push empty hash set for each input cell
            let id = total_cells - input_number;
            let mut set = HashSet::new();
            set.insert(id);
            wirings.push(set);
        }

        self.wiring_pairs.iter().for_each(|(x, y)| {
            wirings.iter_mut().for_each(|set| {
                if set.contains(x) {
                    set.insert(*y);
                } else if set.contains(y) {
                    set.insert(*x);
                }
            })
        });

        Circuit {
            input_config: self.input_config,
            selectors: self.ops,
            total_cells,
            wirings: wirings
                .iter()
                .map(|set| set.iter().copied().collect::<Vec<_>>())
                .collect::<Vec<Vec<_>>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test simple circuit to calculate
    // out = (pub_0 + priv_0) * pub_1 + priv_0
    #[test]
    fn test_build_circuit() {
        let mut builder = CircuitBuilder::new(InputConfig {
            n_public_input: 2,
            n_private_input: 1,
        });

        // First row of addition
        // out_0 = (pub_0 + priv_0)
        let (lhs_0, rhs_0, out_0) = builder.add_addition();

        // Second row of multiplication
        // out_1 = out_0 * pub_1
        let (lhs_1, rhs_1, out_1) = builder.add_multiplication();

        // Last addition
        // out = out_1 + priv_0
        let (lhs_2, rhs_2, out_2) = builder.add_multiplication();

        builder.finalize_gates();

        let pub_ids = builder.get_public_input_ids();
        assert_eq!(pub_ids, vec![]);

        let priv_ids = builder.get_private_input_ids();

        // put wirings as shown above
        builder.add_wire_constraint(pub_ids[0]);
    }
}
