use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    for _ in 0..5 {
        let dice = rng.random_range(1..=6);
        println!("{}", dice);
    }
}