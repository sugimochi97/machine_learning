fn main() {
    let args = std::env::args();
    let mut total = 0.0;
    
    for (i, s) in args.enumerate() {
        if i == 0 { continue; } // 1つ目の引数はプログラム名なのでスキップ
        let num: f64 = match s.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };
        total += num;
    }
    println!("合計値={}", total);
}