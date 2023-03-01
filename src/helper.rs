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

pub fn fibo_gen_vec(init1: u32, init2: u32, limit: u32) -> Vec<u32> {
    let mut series = vec![init1, init2];

    series.sort();

    fibo_gen_mut(&mut series, limit);

    series
}

pub fn fibo_gen_mut(series: &mut Vec<u32>, limit: u32) {
    if series.len() < 2 {
        panic!("Pass two states for fibonacci series.");
    }
    let last_elm = series.get(series.len() - 1).unwrap();
    let second_last_elm = series.get(series.len() - 2).unwrap();
    let sum = last_elm + second_last_elm;

    if sum >= limit {
        return;
    } else {
        series.push(sum);
        fibo_gen_mut(series, limit)
    }
}

pub fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }

    for divisor in 2..num {
        if num % divisor == 0 {
            return false;
        }
        if divisor >= num / divisor {
            break;
        }
    }

    true
}

pub fn prime_gen(num_to: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();

    for num in 2..=num_to {
        if is_prime(num) {
            primes.push(num);
        }
    }

    primes
}

pub fn prime_gen_nth(nth: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    
    let mut num: u64 = 2;

    loop {
        if primes.len() as u64 >= nth {
            break;
        }
        if is_prime(num) {
            primes.push(num);
        }

        num += 1;
    }

    primes
}

pub fn prime_factors_vec(num: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    prime_factors_mut(&mut factors, num);

    factors
}

pub fn prime_factors_mut(factors: &mut Vec<u64>, num: u64) {
    if is_prime(num) && !factors.contains(&num) {
        factors.push(num);
        return;
    }

    for divisor in 2..num {
        if is_prime(divisor) && num % divisor == 0 {
           if !factors.contains(&divisor) {factors.push(divisor)}
           return prime_factors_mut(factors, num / divisor);
        }
    }
}

pub fn is_palindrome(num: u64) -> bool {
    let mut digits: Vec<u64> = Vec::new();
    num_to_digits(&mut digits, num);

    let iterations = ( digits.len() / 2 ) + ( digits.len() % 2 );

    for i in 0..iterations {
        if digits[i] != digits[digits.len() - i - 1] {
            return false
        }
    }
    
    true
}

pub fn num_to_digits(digits: &mut Vec<u64>, num: u64) {
    digits.push(num % 10);
    if num / 10 <= 0 {
        digits.reverse();
        return
    } else {
        num_to_digits(digits, num / 10)
    }
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
    assert_eq!(1, hcf(19, 6));
}

#[test]
fn fibo_gen_test1() {
    assert_eq!(vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89], fibo_gen_vec(2, 1, 90));
}

#[test]
fn prime_check_test1() {
    assert_eq!(false, is_prime(0));
}
#[test]
fn prime_check_test2() {
    assert_eq!(true, is_prime(2));
}
#[test]
fn prime_check_test3() {
    assert_eq!(true, is_prime(3));
}
#[test]
fn prime_check_test4() {
    assert_eq!(false, is_prime(77));
}
#[test]
fn prime_check_test5() {
    assert_eq!(true, is_prime(11));
}

#[test]
fn prime_factors_test1() {
    assert_eq!(vec![] as Vec<u64>, prime_factors_vec(0));
}
#[test]
fn prime_factors_test2() {
    assert_eq!(vec![] as Vec<u64>, prime_factors_vec(1));
}
#[test]
fn prime_factors_test3() {
    assert_eq!(vec![2], prime_factors_vec(2));
}
#[test]
fn prime_factors_test4() {
    assert_eq!(vec![2], prime_factors_vec(4));
}
#[test]
fn prime_factors_test5() {
    assert_eq!(vec![5, 7, 13, 29], prime_factors_vec(13195));
}

#[test]
fn num_to_digits_test1() {
    let mut digits: Vec<u64> = Vec::new();
    let num = 0;
    num_to_digits(&mut digits, num);
    assert_eq!(vec![0], digits);
}
#[test]
fn num_to_digits_test2() {
    let mut digits: Vec<u64> = Vec::new();
    let num = 9;
    num_to_digits(&mut digits, num);
    assert_eq!(vec![9], digits);
}
#[test]
fn num_to_digits_test3() {
    let mut digits: Vec<u64> = Vec::new();
    let num = 123456;
    num_to_digits(&mut digits, num);
    assert_eq!(vec![1,2,3,4,5,6], digits);
}
#[test]
fn num_to_digits_test4() {
    let mut digits: Vec<u64> = Vec::new();
    let num = 1230045000;
    num_to_digits(&mut digits, num);
    assert_eq!(vec![1,2,3,0,0,4,5,0,0,0], digits);
}

#[test]
fn is_palindrom_test1() {
    assert_eq!(true, is_palindrome(0));
}
#[test]
fn is_palindrom_test2() {
    assert_eq!(false, is_palindrome(13));
}
#[test]
fn is_palindrom_test3() {
    assert_eq!(true, is_palindrome(22));
}
#[test]
fn is_palindrom_test4() {
    assert_eq!(true, is_palindrome(12321));
}
#[test]
fn is_palindrom_test5() {
    assert_eq!(false, is_palindrome(123421));
}

#[test]
fn prime_gen_test1() {
    assert_eq!(vec![] as Vec<u64>, prime_gen(1));
}
#[test]
fn prime_gen_test2() {
    assert_eq!(vec![2,3,5,7], prime_gen(10));
}
#[test]
fn prime_gen_test3() {
    assert_eq!(vec![2,3,5,7,11,13,17,19], prime_gen(20));
}
#[test]
fn prime_gen_test4() {
    assert_eq!(vec![2], prime_gen(2));
}
