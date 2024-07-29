use std::{char, collections::VecDeque};

use jisho::{lookup, Entry};

const HIRAGANA: &str = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽぁぃぅぇぉゃゅょゎゕゖゔゕゖゝゞゟ";

pub fn word_finder(search_str: String) -> Vec<Entry> {
    let mut input: VecDeque<char> = search_str.chars().collect();
    let mut results: Vec<Entry> = Vec::new();

    while !input.is_empty() {
        let mut word_buffer = String::new();
        let mut last_result: Option<Entry> = None;
        let mut last_string = String::new();

        // Attempt to build and find words until the input is exhausted
        while let Some(c) = input.pop_front() {
            word_buffer.push(c);
            let lookup_word = word_buffer.clone();
            if let Some(result) = lookup(&lookup_word).first() {
                let result = result.to_owned().clone();
                last_result = Some(result);
                last_string = lookup_word.clone();
            } else {
                input.push_front(c);
                break; // Stop if no more results are found
            }
        }

        if let Some(entry) = last_result {
            if !entry.kanji.is_empty()
                && !last_string.chars().all(|letter| HIRAGANA.contains(letter))
            {
                results.push(entry);
            }
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
    fn lookup_works() {
        let result = word_finder("今日は何をしてるか？\n母と買い物をしていただよ".to_owned());
        println!("{:?}", result)
    }
}
