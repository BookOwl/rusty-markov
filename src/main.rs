#[macro_use]
extern crate clap;
extern crate rand;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use rand::{thread_rng, Rng};

/// Create a type allias for Bigrams so that we don't have to keep retyping the long type.
type Bigrams<'a> = HashMap<(&'a str, &'a str), Vec<&'a str>>;

/// Reads a file and returns the contents as an io::Result<String>
fn read_file(path: &str) -> io::Result<String> {
    // Try to open the file
    let mut f = try!(File::open(path));
    // Create a new string to read the file's contents into
    let mut s = String::new();
    // Try to read the file's contents into s
    try!(f.read_to_string(&mut s));
    // Everything was OK, so return an Ok<String> with the file's contents
    Ok(s)
}

/// Returns a HashMap with all the bigrams generated from the corpus
fn generate_bigrams(corpus: &str) -> Bigrams {
    // Create a HashMap to store the bigrams in
    let mut bigrams = HashMap::new();
    // Create a Vec of words from the corpus by splitting on whitespace
    let words: Vec<_> = corpus.split_whitespace().collect();
    // Iterate over the words and their indexes
    for (i, word1) in words.iter().enumerate() {
        // If we have gotten too far in the vector...
        if i + 2 >= words.len() {
            // ...break out of the loop
            break
        }
        // Get the next two words following word1
        let (word2, word3) = (words[i+1], words[i+2]);
        // If there isn't already an entry for the bigram (word1, word2),
        // create it with a new Vec for it's value and return that vec.
        // Otherwise, we get the Vec that is aleady associated with the bigram.
        (*bigrams.entry((*word1, word2)).or_insert(Vec::new()))
        // Now we take that Vec and push word3 into it.
        .push(word3);
    }
    // Return the bigrams
    bigrams
}

/// Generates a string of amount words from bigrams
fn generate_words<'a>(bigrams: &Bigrams, amount: u32) -> Result<String, &'a str> {
    // Pick a random bigram to start the string with and unpack it to first and second
    let (first, mut second) = **match thread_rng()
                                      .choose(&bigrams.keys().collect::<Vec<_>>()) {
        Some(v) => v,
        None => return Err("No starting words could be found")
    };
    // Create a new String to hold our sentence
    let mut s = String::new();
    // Push the first words
    s.push_str(first);
    s.push(' ');
    s.push_str(second);
    // The current bigram is (first, second)
    let mut current_bigram = (first, second);
    // next is the next word in the sentence.
    let mut next;
    // Generate amount-2 words
    for _ in 0..amount-2 {
        // Push a space to the sentence
        s.push(' ');
        // Get a list of words that can be next
        let nexts = match bigrams.get(&current_bigram) {
            Some(v) => v,
            None => return Err("Couldn't generate next word from bigram")
        };
        // Pick a random word to be next from nexts
        next = match thread_rng().choose(nexts) {
            Some(v) => v,
            None => return Err("No words could be found for next")
        };
        // Push the next word.
        s.push_str(next);
        // The current bigram is now (second, next)
        current_bigram = (second, next);
        // and second is now next
        second = next;
    }
    // We have finished generating the words, so return them inside an Ok
    Ok(s)
}

/// Generates a sentence from bigrams
fn generate_sentence<'a>(bigrams: &Bigrams) -> Result<String, &'a str> {
    // The characters that determine the end of a sentence
    let eos = ['.', '?', '!'];
    // The bigrams that can start a sentence are...
    let starters: Vec<_> = bigrams.keys()
    // ... the ones that have the first word of the bigram start with an uppercase letter
                           .filter(|&bigram| bigram.0.chars().nth(0).unwrap().is_uppercase())
                           .collect();
    // Pick a random bigram to start the sentence with and unpack it to first and second
    let (first, mut second) = **match thread_rng().choose(&starters) {
        Some(v) => v,
        None => return Err("No sentence starters could be found")
    };
    // Create a new String to hold our sentence
    let mut s = String::new();
    // Push the first words
    s.push_str(first);
    s.push(' ');
    s.push_str(second);
    // The current bigram is (first, second)
    let mut current_bigram = (first, second);
    // next is the next word in the sentence.
    let mut next;
    loop {
        // Push a space to the sentence
        s.push(' ');
        // Get a list of words that can be next
        let nexts = match bigrams.get(&current_bigram) {
            Some(v) => v,
            None => return Err("Couldn't generate next word from bigram")
        };
        // Pick a random word to be next from nexts
        next = match thread_rng().choose(nexts) {
            Some(v) => v,
            None => return Err("No words could be found for next")
        };
        // Push the next word.
        s.push_str(next);
        // If the next word ends the sentence...
        if eos.contains(&next.chars().last().unwrap()) {
            // ...then break
            break
        }
        // The current bigram is now (second, next)
        current_bigram = (second, next);
        // and second is now next
        second = next;
    }
    // We have finished generating the sentence, so return it inside an Ok
    Ok(s)
}

/// Generate multiple sentences from bigrams
fn generate_sentences<'a>(bigrams: &Bigrams, amount: u32) -> Result<String, &'a str> {
    let mut s = String::new();
    // Push a new sentence to the string
    s.push_str(&try!(generate_sentence(&bigrams)));
    // Repeat amount-1 times
    for _ in 0..amount-1 {
        // Push a newline
        s.push_str("\n\n");
        // Push a new sentence to the string
        s.push_str(&try!(generate_sentence(&bigrams)));
    }
    Ok(s)
}

/// The main function
fn main() {
    // Create the argument parser and parse the args
    let matches = clap_app!(markov =>
        (version: "1.0")
        (author: "Matthew S. <stanleybookowl@gmail.com>")
        (about: "Generates random text using a Markov chain")
        (@arg CORPUS: +required "Sets the text corpus to use")
        (@arg SENTENCES: -s --sentences +takes_value "Sets how many sentences to generate")
        (@arg WORDS: -w --words +takes_value "Sets how many words to generate")
    ).get_matches();

    // Get the values of the corpus, sentences, and words arguments/
    let path = matches.value_of("CORPUS").unwrap();
    let sentences: u32 = matches.value_of("SENTENCES").map(|n| n.parse().unwrap()).unwrap_or(0);
    let words: u32 = matches.value_of("WORDS").map(|n| n.parse().unwrap()).unwrap_or(0);

    // If the user didn't pass a value for sentences or words...
    if sentences == 0 && words == 0 {
        // ... print an error message and return
        println!("You must pass an argument for either --sentences or --words");
        return
    }
    // If the user passed a value for sentences, we will generate sentences.
    // Otherwise we will generate words
    let generating_sentences = sentences > 0;

    // read the corpus file and assign it to corpus.
    let corpus = match read_file(path) {
        Ok(s) => s,
        Err(e) => {
            println!("Error reading corpus file {:?}: {}", path, e);
            return
        }
    };

    // Generate the bigrams from the corpus
    let bigrams = generate_bigrams(&corpus);

    // If we are generating sentences...
    if generating_sentences {
        // ...then print the generated sentences or an error message
        println!("{}", match generate_sentences(&bigrams, sentences) {
            Ok(s) => s,
            Err(e) => format!("Error: {}", e)
        })
    } else {
        println!("{}", match generate_words(&bigrams, words) {
            Ok(s) => s,
            Err(e) => format!("Error: {}", e)
        });
    }

}
