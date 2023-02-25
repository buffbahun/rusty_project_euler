use crate::helper;

pub fn soln_3(num: u64) -> u64 {
    let factors: Vec<u64> = helper::prime_factors_vec(num);

    match factors.last() {
        Some(&n) => n,
        None => panic!("Enter numbers expect 0 and 1 as they dont have any prime factors!")
    }
}
