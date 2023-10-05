use crate::compile::*;
use crate::int::BfValue;
use crate::tokenize::*;
use std::fmt::Debug;

impl Debug for BfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BfToken {{ {}: {:?} }}", self.pos, self.t_type)
    }
}

impl<N: BfValue> Debug for BfInstruction<N> where N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BfInstruction {{ {}: >{} {:?} }}",
            self.pos, self.offset, self.command
        )
    }
}
