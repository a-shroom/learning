use std::io::prelude::*;
use std::io::{self};
use std::fs::File;
use std::path::Path;

pub fn make_guess(path: &Path, guess: String) -> String {
    let player_guess: Vec<char> = guess.chars().collect();
    let mut words: [String; 2] = [String::new(), String::new()];

    if let Ok(lines) = read_lines(path) {
        for (idx, line) in lines.enumerate() {
            if let Ok(el) = line {
                words[idx] = el;
            }
        }
    };

    let word_vec: Vec<char> = words[0].chars().collect();
    let mut guess_vec: Vec<char> = words[1].chars().collect();

    if player_guess.len() > 1 {
        if player_guess == word_vec {
            return format!("Yes, the word was {}!", vec_to_string(&player_guess));
        };
    };

    let mut res: bool = false;

    for (idx, el) in word_vec.iter().enumerate() {
        if *el == player_guess[0] {
            guess_vec[idx] = *el;
            res = true;
        }
    }

    if res {
        let new_guess: String = format!(
            "{}\n{}",
            vec_to_string(&word_vec),
            vec_to_string(&guess_vec)
        );
        write_file(path, &new_guess);
        return format!("Correct! Letter {} found, current guess:\n{}", player_guess[0], vec_to_string(&guess_vec));
    };
    
    return format!("Incorrect, letter {} is not in the secret word.", player_guess[0])
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/*
fn read_file(path: &Path) -> (String, String) {
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open file: {}", why),
        Ok(file) => file,
    };

    let mut s: String = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read file: {}", why),
        Ok(_) => println!("File read."),
    }

    let mut out: [String; 2] = [String::new(), String::new()];

    for (idx, el) in s.split_whitespace().enumerate() {
        out[idx] = el.to_string();
    };

    (out[0], out[1])
}
*/

fn write_file(path: &Path, input: &String) {
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create file: {}", why),
        Ok(file) => file,
    };

    let _response = match file.write_all(input.as_bytes()) {
        Err(why) => panic!("Couldn't write to file: {}", why),
        Ok(_) => println!("Wrote to file."),
    };
}

fn vec_to_string(vec: &Vec<char>) -> String {
    let mut s: String = String::new();

    for el in vec.iter() {
        s.push(*el);
    };

    s
}
