use std::io::prelude::*;
use std::io::{self};
use std::fs::File;
use std::path::Path;
use rand::Rng;

const RAND_WORDS: &str = "randoms.txt";

pub fn begin_game(path: &Path, word: String) -> String {
    let mut temp: String = String::new();
    let rand_path: &Path = Path::new(RAND_WORDS);

    if word == "random".to_string() {
        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(0, 10);
        if let Ok(lines) = read_lines(rand_path) {
            for (idx, line) in lines.enumerate() {
                if let Ok(el) = line {
                    if idx == x {
                        temp = el;
                    }
                }
            }
        };
    }
    else {
        temp = word;
    };

    let mut guess_string: String = String::new();

    for _i in 0..temp.len() {
        guess_string.push('-');
    };

    write_file(path, &format!("{}\n{}", temp, guess_string));

    return "Started a new game.".to_string();
}

pub fn end_game(path: &Path) -> String {
    write_file(path, &"EMTY".to_string());
    return "Ended game.".to_string();
}

#[allow(unused_assignments)]
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

    if vec_to_string(&word_vec) == "EMTY".to_string() {
        return "There is no game currently running.".to_string();
    };

    if player_guess.len() > 1 {
        let mut res: String = String::new();

        if player_guess == word_vec {
            res = format!("Yes, the word was {}!", &guess);
        }
        else if guess == "end".to_string() {
            res = end_game(path);
        }
        else {
            res = format!("No, '{}' is not the secret word.", guess);
        };
        
        return res;
    };

    let mut res: bool = false;

    for (idx, el) in word_vec.iter().enumerate() {
        if *el == player_guess[0] {
            guess_vec[idx] = *el;
            res = true;
        }
    }

    if guess_vec == word_vec {
        end_game(path);
        return format!("Congratulations, you found the secret word - {}!", vec_to_string(&guess_vec));
    };

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