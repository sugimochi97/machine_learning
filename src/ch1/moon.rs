fn main() {
    let moon = 384400.0;
    let car = 80.0;
    let train = 300.0;

    println!("月まで車で{}日", moon / car / 24.0);
    println!("月まで電車で{}日", moon / train / 24.0);
}