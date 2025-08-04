fn main() {
    for i in 1..10 {
        for j in 1..10 {
            if j == 9 {
                print!("{:3}", i*j);
            } else {
                print!("{:3},", i*j);
            }
        }
        println!("");
    }
}