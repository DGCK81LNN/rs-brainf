use crate::compile::*;
use crate::int::*;
use crate::tokenize::*;

fn main() {
    let a: i8 = BfInt::U64(0xffff_ffff_ffff_ffff).into();
    println!("{a}");

    let b = crate::simple_loop::simple_loop::<i64>(-1029870757353375640_i64 as u64, -1067755531375229826_i64 as u64) as i64;
    println!("{b}");

    return;

    let tokens = tokenize("  +++[>+++++++++++<] >...");
    for token in tokens.iter() {
        println!(
            "{} {}",
            token.pos,
            match token.t_type {
                BfTokenType::Increment => "incr",
                BfTokenType::Decrement => "decr",
                BfTokenType::Input => "inpu",
                BfTokenType::Output => "outp",
                BfTokenType::MoveLeft => "movl",
                BfTokenType::MoveRight => "movr",
                BfTokenType::LoopStart => "lops",
                BfTokenType::LoopEnd => "lope",
                _ => "____",
            }
        );
    }

    let program = compile_from_tokens::<u8>(&tokens).unwrap();
    for instruction in program.iter() {
        println!(
            "{} {}",
            instruction.pos,
            match instruction.command {
                BfCommand::Add(_) => "add",
                _ => "___",
            }
        )
    }
}
