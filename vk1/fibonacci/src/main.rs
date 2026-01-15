fn fibonacci(n: u64) {
    let mut current_num: u128 = 0;
    let mut last_num: u128 = 1;

    for _ in 0..n {
        println!("{current_num}");
        let next_num: u128 = current_num + last_num;
        current_num = last_num;
        last_num = next_num;
    }
}

fn main() {
    fibonacci(100);
}
