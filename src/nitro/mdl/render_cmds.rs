use errors::Result;
use util::cur::Cur;

pub trait Sink {
    /// cur_matrix = matrix_stack[stack_pos]
    fn load_matrix(&mut self, stack_pos: u8) -> Result<()>;
    /// matrix_stack[stack_pos] = cur_matrix
    fn store_matrix(&mut self, stack_pos: u8) -> Result<()>;
    /// cur_matrix = cur_matrix * object_matrices[object_id]
    fn mul_by_object(&mut self, object_id: u8) -> Result<()>;
    /// Blends severals matrices together and stores them on the stack.
    ///
    /// This compilcated command is deferred to the implementer who
    /// will have access to model data like the blend matrices.
    fn blend(&mut self, stack_pos: u8, combination: &[((u8, u8), f64)]) -> Result<()>;
    fn draw(&mut self, mesh_id: u8, material_id: u8) -> Result<()>;
}

pub fn run_commands<S: Sink>(cur: Cur, sink: &mut S) -> Result<()> {
    let mut state = RenderInterpreterState::new();
    state.run_commands(sink, cur)
}

pub struct RenderInterpreterState {
    pub cur_material: u8,
    pub cur_stack_pos: u8,
}

impl RenderInterpreterState {
    pub fn new() -> RenderInterpreterState {
        RenderInterpreterState {
            cur_material: 0,
            cur_stack_pos: 0,
        }
    }

    pub fn run_commands<S: Sink>(&mut self, sink: &mut S, mut cur: Cur) -> Result<()> {
        loop {
            let opcode = cur.next::<u8>()?;
            let num_params = cmd_size(opcode, cur)?;
            let params = cur.next_n_u8s(num_params)?;
            trace!("{:#2x} {:?}", opcode, params);
            match opcode {
                0x00 => {
                    // NOP
                }
                0x01 => {
                    // End render commands
                    return Ok(())
                }
                0x02 => {
                    // unknown
                }
                0x03 => {
                    // Load a matrix from the stack
                    sink.load_matrix(params[0])?;
                }
                0x04 | 0x24 | 0x44 => {
                    // Set the current material
                    self.cur_material = params[0];
                }
                0x05 => {
                    // Draw a mesh
                    sink.draw(params[0], self.cur_material)?;
                }
                0x06 | 0x26 | 0x46 | 0x66 => {
                    // Multiply the current matrix by an object matrix, possibly
                    // loading a matrix from the stack beforehand, and store the
                    // result to a stack location.
                    let object_id = params[0];
                    let _parent_id = params[1];
                    let _dummy = params[2];
                    let (stack_id, restore_id) = match opcode {
                        0x06 => (None,            None),
                        0x26 => (Some(params[3]), None),
                        0x46 => (None,            Some(params[3])),
                        0x66 => (Some(params[3]), Some(params[4])),
                        _ => unreachable!(),
                    };

                    if let Some(restore_id) = restore_id {
                        sink.load_matrix(restore_id)?;
                    }
                    sink.mul_by_object(object_id)?;
                    if let Some(stack_id) = stack_id {
                        self.cur_stack_pos = stack_id;
                    }
                    sink.store_matrix(self.cur_stack_pos)?;
                    self.cur_stack_pos += 1;
                }
                0x09 => {
                    // The current matrix is set to the sum of
                    //   weight * matrix_stack[id0] * blend_matrix[id1]
                    // and stored to the given stack slot. If the blend matrix is
                    //
                    let stack_pos = params[0];
                    let num_terms = params[1] as usize;
                    check!(num_terms <= 4);

                    let mut terms = [((0,0), 0.0); 4];
                    let mut param_idx = 2;
                    for i in 0..num_terms {
                        let id0 = params[param_idx];
                        let id1 = params[param_idx+1];
                        let weight = params[param_idx+2] as f64 / 256.0;

                        terms[i] = ((id0, id1), weight);

                        param_idx += 3;
                    }

                    sink.blend(stack_pos, &terms[0..num_terms])?;
                    self.cur_stack_pos = stack_pos;
                }
                _ => {
                    info!("unknown render command: {:#x} {:?}", opcode, params);
                }
            }
        }
    }
}

fn cmd_size(opcode: u8, cur: Cur) -> Result<usize> {
    let len = match opcode {
        0x00 => 0,
        0x01 => 0,
        0x02 => 2,
        0x03 => 1,
        0x04 => 1,
        0x05 => 1,
        0x06 => 3,
        0x07 => 1,
        0x08 => 1,
        0x09 => {
            // The only variable-length command.
            // 1 byte + 1 byte (count) + count u8[3]s
            2 + 3 * cur.clone().next_n_u8s(2)?[1] as usize
        }
        0x0b => 0,
        0x24 => 1,
        0x26 => 4,
        0x2b => 0,
        0x40 => 0,
        0x44 => 1,
        0x46 => 4,
        0x66 => 5,
        0x80 => 0,
        _ => return Err(format!("unknown render command opcode: {:#x}", opcode).into()),
    };
    Ok(len)
}
