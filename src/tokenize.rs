#[derive(Debug)]
pub enum BfTokenType {
    Increment = 1,
    Decrement,
    Input,
    Output,
    MoveLeft,
    MoveRight,
    LoopStart,
    LoopEnd,
    Debugger,
}

pub struct BfToken {
    pub t_type: BfTokenType,
    pub pos: usize,
}

pub fn tokenize(code: &str) -> Vec<BfToken> {
    let mut tokens = Vec::<BfToken>::new();
    for (pos, char) in code.bytes().enumerate() {
        let token = match char {
            b'+' => BfTokenType::Increment,
            b'-' => BfTokenType::Decrement,
            b',' => BfTokenType::Input,
            b'.' => BfTokenType::Output,
            b'<' => BfTokenType::MoveLeft,
            b'>' => BfTokenType::MoveRight,
            b'[' => BfTokenType::LoopStart,
            b']' => BfTokenType::LoopEnd,
            b'#' => BfTokenType::Debugger,
            _ => continue,
        };
        tokens.push(BfToken {
            t_type: token,
            pos,
        });
    }
    tokens
}
