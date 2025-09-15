fn main() {
    let pr = "猫に小判";

    for c in pr.chars() {
        print!("[{}]", c);
    }

    println!("\n文字数={}字", pr.chars().count());

    let pr_chars: Vec<char> = pr.chars().collect();

    for c in pr_chars.iter() {
        print!("[{}]", c);
    }

    println!("\n文字数={}字", pr_chars.len());
}