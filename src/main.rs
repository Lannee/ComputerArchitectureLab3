fn main() {
    
}

fn prob1(n: u64) -> u64{
    // (1..=n)
    //     .filter(|x| x % 3 == 0 || x % 5 == 0)
    //     .sum()

    let mut sum: u64 = 0;
    let i: u64 = 1;

    loop {

        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
        if i > n {
            break;
        }
    }

    sum
}