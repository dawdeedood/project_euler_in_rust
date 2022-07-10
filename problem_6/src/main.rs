fn main() {
    let n = 100;

    println!("Difference: {}",square_of_sum(n) - sum_of_squares(n));
}

fn sum_of_squares(n: i32) -> i32 {
    let mut sum : i32 = 0;

    for counter in 1..n+1 {
        sum += counter * counter;
    }
    
    sum
}

fn square_of_sum(n : i32) -> i32 {
    let mut sum: i32 = 0;
    for counter in 1..n+1 {
        sum += counter;
    }

    sum * sum
}
