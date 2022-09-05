use std::{io::stdin, char};
//use std::io::stdout;

fn init_words(word:&String, guess_vec:&mut Vec<char>, word_vec:&mut Vec<char>) {
    for _i in 0..word.len() {
        guess_vec.push('_');
    };

    for el in word.chars() {
        word_vec.push(el);
    };
    guess_vec.push('\n');
    word_vec.push('\n');
}

fn print_vec(vector:&Vec<char>) {
    for el in vector.iter() {
        print!("{}", *el);
    };
    println!("");
}

fn resolve_guess(word_vec:&Vec<char>, guess_vec:&mut Vec<char>, guess:&mut String) -> bool {
    let char_guess:Vec<char> = guess.chars().collect();

    if guess.len() > 1 {
        if *guess == "end\n".to_string() {
            print!("Too bad. The word was: ");
            print_vec(&word_vec);
            return true;
        };

        if char_guess == *word_vec {
            print!("Yes! The word was: ");
            print_vec(&word_vec);
            return true;
        };
    };

    for (idx, el) in word_vec.iter().enumerate() {
        if *el == char_guess[0] {
            guess_vec[idx] = *el;
        };
    };

    print!("Current guess: ");
    print_vec(&guess_vec);

    // Reset guess else read_line() keeps appending it instead of replacing it
    *guess = "".to_string();
    return false;
}

fn main() {
    let word:String = String::from("kitten");
    let mut guess_vec:Vec<char> = vec![];
    let mut word_vec:Vec<char> = vec![];

    init_words(&word, &mut guess_vec, &mut word_vec);

    let mut guess:String = String::new();
    let mut _inp = stdin().read_line(&mut guess);

    let mut res:bool = resolve_guess(&word_vec, &mut guess_vec, &mut guess);
    if res {
        return;
    };
    
    while guess_vec != word_vec {
        _inp = stdin().read_line(&mut guess);

        res = resolve_guess(&word_vec, &mut guess_vec, &mut guess);
        if res {
            return;
        };
    };

    print!("Yes! The word was: ");
    print_vec(&word_vec);

}
