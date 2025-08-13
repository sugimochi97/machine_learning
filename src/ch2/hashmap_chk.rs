use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("A", 100);
    map.insert("B", 200);

    if map.get("C") == None {
        println!("Cは存在しない");
    } else {
        println!("{}", map["C"]);
    }
}