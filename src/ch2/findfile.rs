use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("USAGE: findfile (path) (keyword)");
        return;
    }

    let target_dir = &args[1];
    let keyword = &args[2];

    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}

fn findfile(target: &path::PathBuf, keyword: &str) {
    let files = target.read_dir().unwrap();
    for dir_entry in files {
        let path = dir_entry.unwrap().path();

        // ディレクトリなら再帰的に探索する
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }

        let fname = path.file_name().unwrap().to_string_lossy();
        // ファイル名にキーワードが含まれていなければスキップ
        if None == fname.find(keyword) { continue; }

        println!("{}", path.to_string_lossy());
    }
}