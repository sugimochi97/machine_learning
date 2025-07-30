use std::io;

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

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (400, 3.2, 2);
    println!("{}, {}, {}", x.0, x.1, x.2);

    let _months = ["January", "February", "March", "April", "May", "June", "July",
                 "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of the first element is: {}", a[0]);
    println!("The value of the second element is: {}", a[1]);
    println!("The value of the third element is: {}", a[2]);
    println!("The value of the fourth element is: {}", a[3]);
    println!("The value of the fifth element is: {}", a[4]);

    let a: [i32; 5] = [3; 5];
    println!("The value of the first element is: {}", a[0]);
    println!("The value of the second element is: {}", a[1]);
    println!("The value of the third element is: {}", a[2]);
    println!("The value of the fourth element is: {}", a[3]);
    println!("The value of the fifth element is: {}", a[4]);

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );


}
