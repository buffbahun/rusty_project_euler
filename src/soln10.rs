use crate::helper;

pub fn soln_10(num_below: u64) -> u64 {
    helper::prime_gen(num_below - 1).iter().sum()
}
