fn main() {
    let n = 20;

    println!("Smallest Possible Multiple: {}", smallest_possible_multiple(n));
}

fn smallest_possible_multiple(n: i32) -> i32 {
    let mut counter : i32 = n;
    let integer = 'multiple: loop {
        counter += 1;
        for inner_counter in 1..n+1 {
            if (counter % inner_counter != 0) {
                continue 'multiple;
            }
        }
        break counter;
    };
    integer
}
