fn fibonacci(n: u64) {
    let mut current_num: u64 = 0;
    let mut last_num: u64 = 1;

    for _ in 0..n {
        println!("{current_num}");
        let next_num: u64 = current_num + last_num;
        current_num = last_num;
        last_num = next_num;
    }
}

fn main() {
    fibonacci(10);
}
