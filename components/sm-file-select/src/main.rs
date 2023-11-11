use std::path::Path;

mod suggest;

use suggest::{OsFileLister, PathSuggester};

fn try_list(root: &str, relative_path: &str) {
    let mut sg = PathSuggester::new(root, &OsFileLister);

    use std::path::Component;

    for comp in Path::new(relative_path).components() {
        if let Component::Normal(part) = comp {
            sg.push_path(part.to_str().unwrap());
        }
    }

    match sg.suggest_with_strategy_all_nodes() {
        Ok(res) => {
            print!("suggest under `{}`: [", sg.current_path().display());
            let total = 3;
            for (i, pick) in res.iter().take(total).enumerate() {
                print!("{}, ", pick);
                if i == total - 1 {
                    print!("...")
                }
            }
            println!("]");
        }
        Err(err) => {
            println!("err: {}", err)
        }
    }
}

fn main() {
    println!("select file");

    try_list("/home/ian", "");
    try_list("/home/ian", "./go");
    //     try_list("/", "home/ian/g");
    //     try_list("~", ".");
}
