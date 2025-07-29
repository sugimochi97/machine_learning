fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);
    let product = 4 * 30;
    println!("The value of product is: {}", product);
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);
    let floored = 2 / 3;
    println!("The value of floored is: {}", floored);
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);
    let t = ture;
    let f:bool = false;

    
}
