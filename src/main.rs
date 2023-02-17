use std::env;
use std::fs;
use std::convert::TryInto;

fn main() {
    // grep PATH PARAMETER
    let mut sargs: Vec<String> = Vec::new();

    for arg in env::args().skip(1) {
        sargs.push(arg);
    }
    let path = sargs[0].to_string();
    let par: String = sargs[1].to_string();
    let str: String = sargs[2].to_string();
    let mut content: Vec<String> = Vec::new();

    // read dir and push to content
    for element in fs::read_dir(path).unwrap() {
        content.push(element
            .unwrap()
            .path()
            .display()
            .to_string());
    }
    // check parameter and execute command

    // only show elements that CONTAIN str
    if par == "-s" || par == "--contains" {
        for element in content.iter() {
            if element.contains(&str) {
                println!("{}", element);
            }
        }
        // only show elements that DON'T CONTAIN str
    } else if par == "-v" || par == "--invert-match" {
        for element in content.iter() {
            if element.contains(&str) {
                print!("");
            } else {
                println!("{}", element)
            }
        }
        // COUNT how many times str is in function (not case sensitive)
    } else if par == "-c" || par == "--count" {
        let mut count: i32 = 0;
        for element in content.iter() {
            let c: i32 = element
                .to_string()
                .matches(&str)
                .count()
                .try_into()
                .unwrap();
            count += c;
        } println!("{}", count
            .to_string());
    }
}