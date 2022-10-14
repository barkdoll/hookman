extern crate rand;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::io;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter 
{
    character: char,
    revealed: bool,
}

enum GameProgress {
    InProgress,
    Won,
    Lost
}

fn main() 
{
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);
    let mut misses = ALLOWED_ATTEMPTS;

    loop {
        println!("\nYou have {} misses left.", misses);
        println!("Please enter a letter to guess:");

        display_progress(&letters);

        let user_char = read_user_input_character();

        if user_char == '*' {
            break;
        }

        let mut matched_word_char = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                matched_word_char = true;
            }
        }

        if !matched_word_char {
            misses -= 1;
        }

        match check_progress(misses, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats, you saved the hookman!");
                println!("The word was: \"{}\"", selected_word);
                break;
            }
            GameProgress::Lost => {
                println!("\n\nSorry, you lost! The hookman is dead. Sadge.");
                break;
            }
        }
    }
}

fn select_word() -> String 
{
    // Open file
    let mut file = File::open("games.txt")
        .expect("Could not open file!");
    // Load file contents
    let mut file_contents = String::new();
    
    file.read_to_string(&mut file_contents)
        .expect("Error occurred reading games file.");
    
    // Get available words
    let available_words: Vec<&str> = file_contents.trim().split(',').collect();

    // Generate random index
    let random_index = rand::thread_rng().gen_range(0..available_words.len());

    return String::from(available_words[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter> 
{
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false,
        });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>) 
{
    let mut display_string = String::from("Progress:");
    for letter in letters {
        display_string.push(' ');
        if letter.revealed {
            display_string.push(letter.character);
        }
        else {
            display_string.push('_');
        }
    }

    println!("{}", display_string);
}

fn read_user_input_character() -> char 
{
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        Err(_) => { return '*'; }
    }
}

fn check_progress(misses: u8, letters: &Vec<Letter>) -> GameProgress {
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }
    
    if all_revealed {
        return GameProgress::Won;
    }

    if misses > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}