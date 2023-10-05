use crate::int::*;

#[inline]
fn safe_shl(lhs: u64, rhs: u32) -> u64 {
    lhs.checked_shl(rhs).unwrap_or(0)
}

pub fn simple_loop<T: BfValue>(initial: u64, delta: u64) -> u64
{
    let mut loops = 0u64;
    let mut lp = 0u32;
    let bits = std::mem::size_of::<T>() as u32 * 8;

    for ln in 1..=bits {
        let mask = !safe_shl(0xffff_ffff_ffff_ffff, ln);
        let m_delta: u64 = delta & mask;
        if lp == 0 && m_delta != 0 {
            lp = ln
        }
        if (initial.wrapping_add(loops.wrapping_mul(m_delta))) & mask != 0 {
            if lp != 0 {
                loops += 1 << (ln - lp)
            } else {
                return 0;
            }
        }
    }
    loops
}
