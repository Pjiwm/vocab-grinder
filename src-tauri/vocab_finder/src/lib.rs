use std::{char, collections::VecDeque};

use jisho::{lookup, Entry};

pub fn word_finder(search_str: String) -> Vec<Entry> {
    let mut input: VecDeque<char> = search_str.chars().collect();
    let mut results: Vec<Entry> = Vec::new();

    while !input.is_empty() {
        let mut word_buffer = String::new();
        let mut last_result: Option<Entry> = None;

        // Attempt to build and find words until the input is exhausted
        while let Some(c) = input.pop_front() {
            word_buffer.push(c);
            let lookup_word = word_buffer.clone();
            if let Some(result) = lookup(&lookup_word).first() {
                let result = result.to_owned().clone();
                last_result = Some(result);
            } else {
                input.push_front(c);
                break; // Stop if no more results are found
            }
        }

        if let Some(entry) = last_result {
            results.push(entry);
        } else {
            // If no valid word was found, move on to the next character
            input.pop_front();
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = lookup("今日は何をしてるか？\n母と買い物をしていただよ。");
        println!("{:?}", result);
        let result = lookup("今日は");
        println!("{:?}", result);
        let result = lookup("今日");
        println!("{:?}", result);
        let result = lookup("今日は何");
        println!("{:?}", result);
        let result = lookup("は");
        println!("{:?}", result);
    }

    #[test]
    fn lookup_works() {
        let result = word_finder("今日は何をしてるか？\n母と買い物をしていただよ".to_owned());
        println!("{:?}", result)
    }
}
