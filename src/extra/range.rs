fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("入力エラー");
    println!("{}", sum_range(input.trim().parse().expect("数値変換エラー")));
}

fn sum_range(end: i32) -> i32 {
    if end <= 0 {
        return 0;
    }
    return end + sum_range(end - 1);
}