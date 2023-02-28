use project_euler::*;

// use std::time::Instant;

fn main() {
    println!("solution 1: {}", soln1::soln_1(3, 5, 1000));
    println!("solution 2: {}", soln2::soln_2(1, 2, 4_000_000));
    println!("solution 3: {}", soln3::soln_3(600_851_475_143));
    println!("solution 4: {}", soln4::soln_4(100, 999));
    println!("solution 5: {}", soln5::soln_5(20));
    println!("solution 6: {}", soln6::soln_6(100));

    // let start = Instant::now();
    // helper::prime_gen(1000000);
    // let duration = start.elapsed();
    //
    // println!("prime: {:?}", duration);
}
