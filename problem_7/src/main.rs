fn main() {
    let mut counter : u64 = 0;
    let mut prime_counter : u64 = 0; 
    let n : u64 = 10001;
    while prime_counter < n {
        if is_prime_or_not(counter) {
            prime_counter += 1;
        }
        counter += 1;
    }

    println!("The {}th prime number is {}", n, counter - 1);
}

fn is_prime_or_not(n: u64) -> bool {
    let mut counter : u64 = 2;

    while counter < n {
        if n % counter == 0 {
           break;
        }
        counter += 1;
    }

    if counter == n {
        return true;
    } else {
        return false;
    }
}
