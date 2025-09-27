use dashu_int::UBig;

fn main() {
    const IS: &str = "is";
    const IS_NOT: &str = "is not";

    const STOP: u32 = 2_000;
    for n in 2..=STOP {
        let is_prime = simple_factor(n);
        let result_str = if is_prime.0 { IS } else { IS_NOT };
        println!("{n} {result_str} prime.");

        if is_prime.0 {
            let is_prime = lucas_lehmer(n);
            let result_str = if is_prime.0 { IS } else { IS_NOT };
            println!("  2^{n} - 1 {result_str} a Mersenne prime.");
        }
    }
}

struct IsPrime(bool);

impl From<bool> for IsPrime {
    fn from(value: bool) -> Self {
        IsPrime(value)
    }
}

fn lucas_lehmer(n: u32) -> IsPrime {
    let mut s = UBig::from(4_u32);
    let m = UBig::from(2_u32).pow(n.try_into().unwrap()) - 1_u32;
    for _ in 0..n - 2 {
        s = (&s * &s - 2_u32) % &m;
    }
    if s == UBig::from(0_u32) {
        IsPrime(true)
    } else {
        IsPrime(false)
    }
}

fn simple_factor(n: u32) -> IsPrime {
    if n < 2 {
        return false.into();
    } else if n == 2 {
        return true.into();
    } else if n % 2 == 0 {
        return false.into();
    }

    let mut m = 3;
    loop {
        if m * m > n {
            break;
        }
        if n % m == 0 {
            return false.into();
        }
        m += 2;
    }

    true.into()
}
