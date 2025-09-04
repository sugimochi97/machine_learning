struct CarSpec {
    model: i32,
    cc: i32,
    color: i32,
}

fn main() {
    let car1 = CarSpec {
        model: 1001,
        cc: 1000,
        color: 0x0000FF,
    };
    let car2 = CarSpec {
        model: 1002,
        cc: 2000,
        color: 0x00FF00,
    };

    println!("car1: model={}, cc={}, color={:06x}", car1.model, car1.cc, car1.color);
    println!("car2: model={}, cc={}, color={:06x}", car2.model, car2.cc, car2.color);
}