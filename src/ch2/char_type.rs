fn main() {
    let a = 'a';
    let b = b'A';
    let c = '\x61';
    println!("{}", a);
    println!("{:2x}", b);
    println!("{}", c);

    let d = '愛';
    let e = '愛' as u32;
    let f = '\u{611b}';
    println!("{}", d);
    println!("{:4x}", e);
    println!("{}", f);
}