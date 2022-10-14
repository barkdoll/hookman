extern crate rand;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let selected_word = select_word();
    println!("The selected word is: \"{}\"", selected_word);
}

fn select_word() -> String {
    // Open file
    let mut file = File::open("games.txt").expect("Could not open file!");
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