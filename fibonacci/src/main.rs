use std::io;

fn main() {
    println!("몇 번째 피보나치 수를 알고 싶은 지 입력하세요.");

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let input = input.trim().parse().expect("Please input the number");
    let num = calculate_fibonacci(input);
    println!("{input}번째 피보나치 수는 {num} 입니다.")
}

fn calculate_fibonacci(order: u32) -> u32 {
    match order {
        1 => 1,
        2 => 1,
        _ => calculate_fibonacci(order - 1) + calculate_fibonacci(order - 2)
    }
}
