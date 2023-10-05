use crate::int::*;
use crate::kvvec::*;
use crate::tokenize::*;

#[derive(Debug)]
pub enum BfCommand<N: BfValue> {
    Add(Box<[(i32, N)]>),
    Input(),
    Output(Box<[(i32, N)]>),
    SimpleLoop(Box<[(i32, N)]>),
    LoopStart(usize),
    LoopEnd(usize),
    Debugger(),
}

//#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug)]
pub enum CompileError {
    UnmatchedBrackets,
}

pub struct BfInstruction<N: BfValue> {
    pub command: BfCommand<N>,
    pub offset: i32,
    pub pos: usize,
}

fn flush_outs<N: BfValue>(
    program: &mut Vec<BfInstruction<N>>,
    output_buf: KVVec<i32, N>,
    pos: usize,
) -> KVVec<i32, N> {
    if output_buf.len() == 0 {
        return output_buf;
    }
    program.push(BfInstruction {
        command: BfCommand::Output(output_buf.into_boxed_slice()),
        offset: 0,
        pos,
    });
    KVVec::new()
}

fn flush_adds<N: BfValue>(
    program: &mut Vec<BfInstruction<N>>,
    add_buf: KVVec<i32, N>,
    pos: usize,
) -> KVVec<i32, N> {
    if add_buf.len() == 0 {
        return add_buf;
    }
    program.push(BfInstruction {
        command: BfCommand::Add(add_buf.into_boxed_slice()),
        offset: 0,
        pos,
    });
    KVVec::new()
}

pub fn compile_from_tokens<N: BfValue>(
    tokens: &Vec<BfToken>,
) -> Result<Box<[BfInstruction<N>]>, CompileError> {
    let mut program = Vec::<BfInstruction<N>>::new();
    let mut add_buf = KVVec::<i32, N>::new();
    let mut output_buf = KVVec::<i32, N>::new();
    let mut loop_stack = Vec::<usize>::new();

    let mut offset = 0i32;
    let mut buf_i = 0usize;

    for (i, token) in tokens.iter().enumerate() {
        let pos = token.pos;
        match token.t_type {
            BfTokenType::Increment => {
                add_buf.incr(offset);
            }
            BfTokenType::Decrement => {
                add_buf.decr(offset);
            }
            BfTokenType::Input => {
                output_buf = flush_outs(&mut program, output_buf, buf_i);
                add_buf = flush_adds(&mut program, add_buf, buf_i);
                program.push(BfInstruction {
                    command: BfCommand::Input(),
                    offset,
                    pos,
                });
                offset = 0;
                buf_i = i + 1;
            }
            BfTokenType::Output => {
                output_buf.push(offset, add_buf.get(offset));
            }
            BfTokenType::MoveLeft => {
                offset -= 1;
            }
            BfTokenType::MoveRight => {
                offset += 1;
            }
            BfTokenType::LoopStart => {
                output_buf = flush_outs(&mut program, output_buf, buf_i);
                add_buf = flush_adds(&mut program, add_buf, buf_i);
                loop_stack.push(program.len());
                program.push(BfInstruction {
                    command: BfCommand::LoopStart(0),
                    offset,
                    pos,
                });
                offset = 0;
                buf_i = i + 1;
            }
            BfTokenType::LoopEnd => {
                let target = loop_stack.pop();
                if target.is_none() {
                    return Err(CompileError::UnmatchedBrackets);
                }
                let target = target.unwrap();
                let program_len = program.len();
                let instruction = &mut program[target];
                if output_buf.len() == 0 && offset == 0 && target == program_len - 1 {
                    instruction.command = BfCommand::SimpleLoop(add_buf.into_boxed_slice());
                    add_buf = KVVec::new();
                } else {
                    instruction.command = BfCommand::LoopStart(program_len);
                    output_buf = flush_outs(&mut program, output_buf, buf_i);
                    add_buf = flush_adds(&mut program, add_buf, buf_i);
                    program.push(BfInstruction {
                        command: BfCommand::LoopEnd(target),
                        offset,
                        pos,
                    });
                    offset = 0;
                }
                buf_i = i + 1;
            }
            BfTokenType::Debugger => {
                output_buf = flush_outs(&mut program, output_buf, buf_i);
                add_buf = flush_adds(&mut program, add_buf, buf_i);
                loop_stack.push(program.len());
                program.push(BfInstruction {
                    command: BfCommand::Debugger(),
                    offset,
                    pos,
                });
                offset = 0;
                buf_i = i + 1;
            }
        }
    }

    if loop_stack.len() != 0 {
        return Err(CompileError::UnmatchedBrackets);
    }
    flush_outs(&mut program, output_buf, buf_i);

    Ok(program.into_boxed_slice())
}
