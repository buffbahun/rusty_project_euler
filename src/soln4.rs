use crate::helper;

pub fn soln_4(lower: u64, upper: u64) -> u64 {
    let mut palindrome: u64 = 0;
    for i in (lower..=upper).rev() {
        for j in (lower..=i).rev() {
            let product = i * j;

            if helper::is_palindrome(product) && product > palindrome {
                palindrome = product;
            }

            if product <= palindrome {
                break;
            }
        }
    }

    palindrome
}
