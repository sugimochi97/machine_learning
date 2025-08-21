fn main () {
    {
        let s1 = String::from("穏やかな心は体に良い");
        let s3 = String::from("ありがとう");

        {
            let s2 = s1;
            println!("{}", s2);
        }
        println!("{}", s3);
    }
}