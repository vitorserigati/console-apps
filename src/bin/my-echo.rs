use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        parse_args(args);
    } else {
        eprintln!("No argument provided!");
        process::exit(1);
    }
}

fn parse_args(text: Vec<String>) -> String {
    let text = text[1].clone();
    println!("{}", text);
    return text;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_right_text() {
        let result = parse_args(vec!["path".to_string(), "texto1".to_string()]);
        assert_eq!(result, "texto1".to_string());
    }

    #[test]
    #[should_panic]
    fn it_panics_if_no_args_given() {
        _ = parse_args(vec!["path".to_string()]);
    }
}
