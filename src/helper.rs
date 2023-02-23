pub fn hcf(num1: u32, num2: u32) -> u32 {
    let gtr = if num1 > num2 {num1} else {num2};
    let sml = if gtr == num1 {num2} else {num1};

    if sml == 0 && gtr == 0 {
        panic!("(O, O) HCF undefined behaviour!");
    }

    if sml == 0 {
        return gtr;
    }

    if gtr % sml == 0 {
        sml
    } else {
        hcf(sml, gtr - sml)
    }
}

pub fn lcm(num1: u32, num2: u32) -> u32 {
    let hcf = hcf(num1, num2);

    (num1 * num2) / hcf
}

pub fn sum_of_num(n: u32) -> u32 {
    ( (n + 1) * n ) / 2
}

// unit test functions --------------------
#[test]
fn hcf_test1() {
    assert_eq!(5, hcf(0, 5));
}

#[test]
fn hcf_test2() {
    assert_eq!(1, hcf(1, 0));
}

#[test]
#[should_panic]
fn hcf_test3() {
    hcf(0, 0);
}

#[test]
fn hcf_test4() {
    assert_eq!(3, hcf(6, 9));
}

#[test]
fn hcf_test5() {
    assert_eq!(1, hcf(6, 19));
}
