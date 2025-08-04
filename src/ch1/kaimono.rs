fn main() {
    let pc_price = 98000.0;
    let a_rate = 0.2;
    let a_ship_fee = 1200.0;
    let b_rate = 0.1;
    let b_ship_fee = 0.0;

    println!("A社のPCを買うと、{}円", pc_price * (1.0-a_rate) + a_ship_fee);
    println!("B社のPCを買うと、{}円", pc_price * (1.0-b_rate) + b_ship_fee);
}