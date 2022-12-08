/// The Hangman Game

/// Allows Random Numbers To Be Generated
extern crate rand;
use rand::Rng;

/// Helps With All File Handling
use std::fs::File;
use std::io::prelude::*;

/// Handles Any User Input When Running The Project
use std::io;

/// Gives User 5 Turns When Playing, Which Decrements When User Inputs Incorrect Letter
const ALLOWED_ATTEMPTS: u8 = 5;

/// The Letter The User Enters Is A Single Character, Which Is Either Right Or Wrong Based On The Word Selected
struct Letter {
    character: char,
    revealed: bool
}

/// The OutCome Options When The Game Is Executed
enum GameProgress {
    InProgress,
    Won,
    Lost
}

/// Main Method - Implements The User-Input Version Of Hangman
fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);

    println!("Welcome to Hangman!");   /// Intro Into The Game

    /// The Loop Repeats With Each Letter The User Inputs
    /// The Turns Only Decrease If The Character Entered By The User Is Incorrect
    /// The Loop Ends When There Are No Turns Left
    loop {
        println!("\nYou have {} turns left.", turns_left);
        display_progress(&letters);   /// Shows Word Updated With The Letters Correctly Guessed

        /// Prompts User To Enter New Character To Guess
        println!("\nPlease enter a letter to guess:");
        let user_char = read_user_input_character();

        /// Exit If User Enters An Asterisk ('*') - Allows User To Stop The Game Prematurely
        if user_char == '*' {
            break;
        }

        /// Updates The 'Revealed' State Of Each Letter. If The User Has Guessed A Correct Letter, at_least_one_revealed Is Changed To True
        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        /// The User Will Lose A Turn If You Make An Incorrect Guess
        if !at_least_one_revealed {
            turns_left -= 1;
        }

        /// Check Game Progress
        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats! You won! ☺");  /// Prints With Smiley Face If Entire Word Is Correctly Guessed
                break;
            }
            GameProgress::Lost => {
                println!("\nYou lost! ☹");   /// Prints With Sad Face If Word Isn't Guessed Before The User Runs Out Of Turns
                break;
            }
        }
    }

    println!("\nGoodbye!");
}

/// Open The File Containing List Of Words And Select One At Random, Returning It As A String
fn select_word() -> String {
    let mut file = File::open("words.txt").expect("Could not open file!");   /// Prints If File Cannot Be Opened

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("An error occured while reading the file!");   /// Prints If Error Occurs In File When Executing Project

    let available_words: Vec<&str> = file_contents.split(',').collect();

    /// Select A Word At Random From The Included File
    let random_index = rand::thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);   /// Returns The Selected Word From The File
}

/// Given A Word (Type String), Create A Vector Of Letter's From It With Default Members And Return It
fn create_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    /// Doesn't Reveal Letters, Keeps The Letters Blank In Display Unless Character Is Correctly Guessed
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }

    return letters;   /// Returns The Letter That Is Guessed
}

/// Displays the progress of the game based off Vec<Letter>
fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");  /// Updated Word With New Letters Revealed If User Correctly Guesses During Their Turn

    for letter in letters {
        display_string.push(' ');

        /// Loops Through To Push Underscores For Letters In Word, Replacing With Its Letter If User Guesses It Correctly
        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);   /// Updated String From Word Being Guesses
}

/// Reads A Character From User Input. If Multiple Characters Are Given, Character At First Index Is Returned
/// In Any Problematic Cases, Return An Asterisk (*)
fn read_user_input_character() -> char {
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

/// Checks The User's Progress In The Game And Returns The Appropriate GameProgress Member
fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    /// Determine If All Letters Have Been Revealed
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    /// If All Of The Letters In The Word Are Correctly Guessed And Revealed, The User Won The Game
    if all_revealed {
        return GameProgress::Won;
    }

    /// If The User Still Has Turns Left And The Word Is Not Yet Revealed, The Game Is Still In Progress
    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    /// If The Letters In The Word Are Not All Correctly Guessed And Revealed Before Their Turns Run Out, The User Lost The Game
    return GameProgress::Lost;
}
