// 要件
// 奇数だけを出力する
// 7の倍数のときに「Lucky」と表示する

fn main() {
    to_100(1);
}

fn to_100(start: i32){
    if start % 7 == 0 {
        println!("Lucky");
    } else if start % 2 == 1{
        println!("{}", start);
    }
        
    if start <= 100 {
        println!("{}", start);
    }
    to_100(start+1);
}
