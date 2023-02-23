pub mod helper;

pub fn soln_1(num1: u32, num2: u32, limit: u32) -> u32 {
    let sumto_num1 = (limit - 1) / num1;
    let sumto_num2 = (limit - 1) / num2;

    let lcm = helper::lcm(num1, num2);
    let sumto_lcm = (limit - 1) / lcm;

    num1 * helper::sum_of_num(sumto_num1) + num2 * helper::sum_of_num(sumto_num2) - lcm * helper::sum_of_num(sumto_lcm)
}


