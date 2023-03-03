pub fn soln_9() -> u64 {
    let mut a: u64 = 1;
    loop {
        if ( 1000 * (500 - a) ) % (1000 - a) != 0 {
            a += 1;
            continue;
        }

        let b = ( 1000 * (500 - a) ) / (1000 - a);
        let c = 1000 - a - b;

        if c > b && b > a {
            return a * b * c;
        }

    }
}
