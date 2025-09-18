fn main() {
    let s = "苦しむ人にはどの日も悪い日である。";
    let s2 = s.replace("悪い日", "良い日");
    let s3 = s2.replace("苦しむ人", "陽気な人");
    println!("置換前: {}", s);
    println!("置換後: {}", s3);
}