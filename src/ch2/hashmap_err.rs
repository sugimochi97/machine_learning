use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("A", 100);
    map.insert("B", 200);

    let d = map["D"];
    println!("{}", d);
}