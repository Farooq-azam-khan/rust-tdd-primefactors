fn main() {
    println!("Hello, world!");
}

#[warn(dead_code)]
fn prime_factors(n: &mut u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    let mut divisor: u32 = 2;
    while *n > 1 {
        while *n % divisor == 0 {
            v.push(divisor);
            *n = *n / divisor;
        }
        divisor += 1;
    }
    v
}

#[test]
fn test_prime_factors_of_1() {
    assert_eq!(prime_factors(&mut 1), Vec::new())
}

#[test]
fn test_prime_factors_of_2() {
    assert_eq!(prime_factors(&mut 2), vec![2])
}

#[test]
fn test_prime_factors_of_3() {
    assert_eq!(prime_factors(&mut 3), vec![3])
}

#[test]
fn test_prime_factors_of_4() {
    assert_eq!(prime_factors(&mut 4), vec![2, 2])
}

#[test]
fn test_prime_factors_of_5() {
    assert_eq!(prime_factors(&mut 5), vec![5])
}

#[test]
fn test_prime_factors_of_6() {
    assert_eq!(prime_factors(&mut 6), vec![2, 3])
}

#[test]
fn test_prime_factors_of_8() {
    assert_eq!(prime_factors(&mut 8), vec![2, 2, 2])
}

#[test]
fn test_prime_factors_of_9() {
    assert_eq!(prime_factors(&mut 9), vec![3, 3])
}

#[test]
fn test_prime_factors_of_10() {
    assert_eq!(prime_factors(&mut 10), vec![2, 5])
}

#[test]
fn large_prime_factorization() {
    assert_eq!(
        prime_factors(&mut (2 * 2 * 2 * 2 * 3 * 5 * 11 * 13)),
        vec![2, 2, 2, 2, 3, 5, 11, 13]
    )
}
