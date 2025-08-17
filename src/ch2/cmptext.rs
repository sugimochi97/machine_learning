use std::fs;
fn main() {
    let afile = "./fizzbuzz_python.txt";
    let bfile = "./fizzbuzz_python.txt";
    // let bfile = "./fizzbuzz_rust.txt";
    
    let astr = fs::read_to_string(afile).unwrap();
    let bstr = fs::read_to_string(bfile).unwrap();

    let astr = astr.trim();
    let bstr = bstr.trim();

    if astr == bstr {
        println!("ファイルの内容は一致しています。");
    } else {
        println!("ファイルの内容は一致していません。");
    }
}