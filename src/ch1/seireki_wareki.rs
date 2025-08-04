fn main() {
    for year in 1926..2027 {
        if year >= 2019 { 
            if year == 2019 {
                println!("{}年は令和元年です。", year);
            } else {
                println!("{}年は令和{}年です。", year, year - 2019 + 1);
            }
        } else if year >= 1989 {
            if year == 1989 {
                println!("{}年は平成元年です。", year);
            } else {
                println!("{}年は平成{}年です。", year, year - 1989 + 1);
            } 
        } else {
            if year == 1926 {
                println!("{}年は昭和元年です。", year);
            } else {
                println!("{}年は昭和{}年です。", year, year - 1926 + 1);
            }
        }
    }
}