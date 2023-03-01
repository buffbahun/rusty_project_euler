use crate::helper;

pub fn soln_7(nth: u64) -> u64 {
    match helper::prime_gen_nth(nth).last() {
        Some(&n) => n,
        None => panic!("Enther nth term greater than 0.")
    }
}
