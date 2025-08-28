fn main() {
    let banana = ("バナナ", 300);
    let apple = ("リンゴ", 200);
    let total = banana.1 + apple.1;

    print_tuple(&banana);
    print_tuple(&apple);
    println!("Total: {}", total);
}

fn print_tuple(tuple: &(&str, i32)) {
    println!("{}: {}", tuple.0, tuple.1);
}
