use anyhow::{anyhow, Result};
use std::collections::HashSet;

/*
- Define private/public inputs with input configuration.
- Represent wire with column and row
- To add a single gate, you add one row to the computation trace table
  by turning your selector column 0 or 1. 0 for addition and 1 for multiplication.
- When cells are handled by prover, table will be flattened to a single vector.
*/

// In what position of the polynomial the cell exists.
// In order to represent both intermediate cells and input cells,
// For input cells, id: total cell - input number
// For intermediate cells, id:
type Id = usize;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    Add = 1,
    Mul = 0,
}

#[derive(Clone, Copy, Debug)]
pub struct InputConfig {
    n_pub: usize,
    n_priv: usize,
}

impl InputConfig {
    /// Generate new input config.
    pub fn new(n_pub: usize, n_priv: usize) -> Self {
        Self { n_pub, n_priv }
    }

    /// Returns number of public input.
    pub fn n_pub(&self) -> usize {
        self.n_pub
    }

    /// Returns number of private input.
    pub fn n_priv(&self) -> usize {
        self.n_priv
    }

    /// Returns total number of input.
    pub fn total_input(&self) -> usize {
        self.n_pub + self.n_priv
    }
}

/// Circuit struct
#[derive(Clone)]
pub struct Circuit {
    pub(crate) input_config: InputConfig,
    pub(crate) selectors: Vec<Op>,
    copy_constraints: Vec<Vec<Id>>,

    /// Total number of cells including.
    /// gate constraints cells: lhs, rhs, out.
    /// input cells: public inputs, private inputs.
    n_cells: usize,

    /// Total number of rows.
    n_rows: usize,

    /// The last cell id of computation trace table.
    /// Circuit allows single output.
    output: Id,
}

impl Circuit {
    /// Returns total number of inputs.
    pub fn n_inputs(&self) -> usize {
        self.input_config.total_input()
    }

    /// Returns the total number of cells.
    pub fn n_cells(&self) -> usize {
        self.n_cells
    }

    /// Returns the total number of rows.
    pub fn n_rows(&self) -> usize {
        self.n_rows
    }

    /// Returns the id of output cell.
    pub fn output_id(&self) -> Id {
        self.output
    }

    /// Very naive way to retrieve set of cell ids share same value(copy constraints).
    pub fn get_copy_constraints(&self, id: Id) -> Option<&[Id]> {
        self.copy_constraints
            .iter()
            .find(|v| v.contains(&id))
            .map(|v| v.as_slice())
    }

    pub fn get_selector(&self, row: usize) -> Option<Op> {
        self.selectors.get(row).copied()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cellref {
    Input(Id),
    Wire(Id),
}

pub struct CircuitBuilder {
    current_row: usize,
    ops: Vec<Op>,
    /// Store which two cells are equal
    wiring_pairs: Vec<(Cellref, Cellref)>,
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

    /// Returns pair of vec of input refs.
    /// First item is public inputs' refs and second item is private inputs' refs.
    pub fn get_input_refs(&self) -> (Vec<Cellref>, Vec<Cellref>) {
        let pb_len = self.input_config.n_pub();
        let prv_len = self.input_config.n_priv();

        let pb = (1..=pb_len + 1).map(Cellref::Input).collect::<Vec<_>>();
        let prv = (pb_len + 1..=pb_len + prv_len)
            .map(Cellref::Input)
            .collect::<Vec<_>>();

        (pb, prv)
    }

    /// Add new addition gate constraint to a circuit.
    pub fn add_addition(&mut self, lhs: Cellref, rhs: Cellref) -> Result<Cellref> {
        self.validate_cell_ref(lhs)
            .map_err(|e| anyhow!(format!("LHS: {}", e)))?;
        self.validate_cell_ref(rhs)
            .map_err(|e| anyhow!(format!("RHS: {}", e)))?;

        self.ops.push(Op::Add);
        let pos = self.current_row * 3;
        self.current_row += 1;

        // add wiring constraints
        let new_lhs = Cellref::Wire(pos);
        let new_rhs = Cellref::Wire(pos + 1);

        self.add_wire_constraint(lhs, new_lhs);
        self.add_wire_constraint(rhs, new_rhs);

        Ok(Cellref::Wire(pos + 2))
    }

    /// Add new multiplication gate constraint to a circuit.
    pub fn add_multiplication(&mut self, lhs: Cellref, rhs: Cellref) -> Result<Cellref> {
        self.validate_cell_ref(lhs)
            .map_err(|e| anyhow!(format!("LHS: {}", e)))?;
        self.validate_cell_ref(rhs)
            .map_err(|e| anyhow!(format!("RHS: {}", e)))?;

        self.ops.push(Op::Mul);
        let pos = self.current_row * 3;
        self.current_row += 1;

        // add wiring constraints
        let new_lhs = Cellref::Wire(pos);
        let new_rhs = Cellref::Wire(pos + 1);

        self.add_wire_constraint(lhs, new_lhs);
        self.add_wire_constraint(rhs, new_rhs);

        Ok(Cellref::Wire(pos + 2))
    }

    /// Add wire constraint to a circuit.
    pub fn add_wire_constraint(&mut self, x: Cellref, y: Cellref) {
        self.wiring_pairs.push((x, y))
    }

    fn validate_cell_ref(&self, cell: Cellref) -> Result<()> {
        let n_input = self.input_config.total_input();
        match cell {
            Cellref::Input(x) => {
                if x == 0 || x > n_input {
                    return Err(anyhow!("Input {} does not exist.", x));
                }
            }
            Cellref::Wire(x) => {
                if x >= self.current_row * 3 {
                    return Err(anyhow!("Wire {} does not exist.", x));
                }
            }
        };

        Ok(())
    }

    pub fn build(self) -> Result<Circuit> {
        let n_input = self.input_config.total_input();
        let n_cells = n_input + self.current_row * 3;

        // calculate every wirings
        let mut wirings = Vec::with_capacity(n_input);
        for input_number in 1..=n_input {
            // push empty hash set for each input cell
            let id = n_cells - input_number;
            let mut set = HashSet::new();
            set.insert(id);
            wirings.push(set);
        }

        self.wiring_pairs.iter().for_each(|(x_ref, y_ref)| {
            let x = match x_ref {
                Cellref::Wire(x) => *x,
                Cellref::Input(x) => n_cells - x,
            };
            let y = match y_ref {
                Cellref::Wire(y) => *y,
                Cellref::Input(y) => n_cells - y,
            };

            if let Some(wire_set) = wirings.iter_mut().find(|set| set.contains(&x)) {
                wire_set.insert(y);
            } else if let Some(wire_set) = wirings.iter_mut().find(|set| set.contains(&y)) {
                wire_set.insert(x);
            } else {
                let mut set = HashSet::new();
                set.insert(x);
                set.insert(y);
                wirings.push(set);
            }
        });

        let output = self.current_row * 3 - 1;

        Ok(Circuit {
            input_config: self.input_config,
            selectors: self.ops,
            n_cells,
            n_rows: self.current_row,
            copy_constraints: wirings
                .iter()
                .map(|set| {
                    let mut v = set.iter().copied().collect::<Vec<_>>();
                    v.sort();
                    v
                })
                .collect::<Vec<Vec<_>>>(),
            output,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test simple circuit to calculate
    // out = (pub_0 + priv_0) * pub_1 + priv_0
    //
    // Table defined as
    //
    // Input Cells
    // | pub_0 | pub_1 | priv_0 |
    //
    // Wire Cells
    // | lhs   | rhs    | out   | s |
    // |-------|--------|-------|---|
    // | pub_0 | priv_0 | out_0 | 0 |
    // | out_0 | pub_1  | out_1 | 1 |
    // | out_1 | priv_0 | out   | 0 |
    // Inputs
    // pub: 2, priv: 1
    #[test]
    fn test_build_circuit() {
        let mut builder = CircuitBuilder::new(InputConfig::new(2, 1));

        let (pb_refs, prv_refs) = builder.get_input_refs();

        // First row of addition
        // out_0 = (pub_0 + priv_0)
        let out_0 = builder.add_addition(pb_refs[0], prv_refs[0]).unwrap();

        // Second row of multiplication
        // out_1 = out_0 * pub_1
        let out_1 = builder.add_multiplication(out_0, pb_refs[1]).unwrap();

        // Last addition
        // out = out_1 + priv_0
        let _ = builder.add_addition(out_1, prv_refs[0]).unwrap();

        let builder_result = builder.build();
        assert!(builder_result.is_ok(), "Build should succeed.");

        let circ = builder_result.unwrap();
        assert!(
            circ.selectors.eq(&vec![Op::Add, Op::Mul, Op::Add]),
            "Selector cells should be defined correctly."
        );

        // Test wiring constraints
        // Wirings: [[0, 11],[4,10],[1,7,9],[2,3],[5,6]]
        assert!(
            circ.copy_constraints.eq(&vec![
                vec![0, 11],
                vec![4, 10],
                vec![1, 7, 9],
                vec![2, 3],
                vec![5, 6],
            ]),
            "Circuit wirings should be defined correctly."
        );
    }

    #[test]
    fn test_lhs_invalid_input_ref() {
        let mut builder = CircuitBuilder::new(InputConfig::new(1, 0));

        let res = builder.add_addition(Cellref::Input(100), Cellref::Input(1));
        let error = res.unwrap_err();
        assert_eq!(format!("{}", error), "LHS: Input 100 does not exist.");
    }

    #[test]
    fn test_rhs_invalid_input_ref() {
        let mut builder = CircuitBuilder::new(InputConfig::new(1, 0));

        let res = builder.add_addition(Cellref::Input(1), Cellref::Input(100));
        let error = res.unwrap_err();
        assert_eq!(format!("{}", error), "RHS: Input 100 does not exist.");
    }

    #[test]
    fn test_lhs_invalid_wire_ref() {
        let mut builder = CircuitBuilder::new(InputConfig::new(1, 0));

        let res = builder.add_addition(Cellref::Wire(1), Cellref::Input(1));
        let error = res.unwrap_err();
        assert_eq!(format!("{}", error), "LHS: Wire 1 does not exist.");
    }

    #[test]
    fn test_rhs_invalid_wire_ref() {
        let mut builder = CircuitBuilder::new(InputConfig::new(1, 0));

        let res = builder.add_addition(Cellref::Input(1), Cellref::Wire(1));
        let error = res.unwrap_err();
        assert_eq!(format!("{}", error), "RHS: Wire 1 does not exist.");
    }
}
