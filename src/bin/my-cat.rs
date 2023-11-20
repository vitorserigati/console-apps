use std::{env, fs, process};

fn read_file_path(path: &Vec<String>) -> String {
    let output = path[1].clone();
    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        eprintln!("No file provided");
        process::exit(1);
    }

    let file_path = read_file_path(&args);
    let content = match fs::read_to_string(&file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error! {} {}. Should have provided a file path!", &file_path, e);
            process::exit(1);
        }
    };

    println!("{}", content);
}

#[cfg(test)]
mod cat_test {
    use super::*;

    #[test]
    fn it_searchs_for_given_path() {
        let path = read_file_path(&vec!["path".to_string(), "./test/path".to_string()]);

        assert_eq!(path, "./test/path".to_string());
    }
}
