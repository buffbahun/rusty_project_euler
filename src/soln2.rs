use crate::helper;

pub fn soln_2(init1: u32, init2: u32, limit: u32) -> u32 {
    let fibo_series = helper::fibo_gen_vec(init1, init2, limit);

    let even_fibo_sum = fibo_series.iter().filter(|num| *num % 2 == 0).sum();

    even_fibo_sum
}

