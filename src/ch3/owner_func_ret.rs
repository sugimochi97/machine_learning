fn main() {
    let mut g1 = String::from("過ちを見過ごす人は美しい");

    g1 = show_message(g1);
    println!("{}", g1);
}

fn show_message(msg: String) -> String {
    println!("{}", msg);
    return msg;
}