use jisho::{lookup, Entry};
use serde::Deserialize;
use std::{
    char,
    collections::{HashMap, VecDeque},
    hash::Hash,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
};

const HIRAGANA: &str = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽぁぃぅぇぉゃゅょゎゕゖゔゕゖゝゞゟ";

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct DictEntry(Entry);

impl Eq for DictEntry {}

impl Hash for DictEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.reading.hash(state);
        self.0.kanji.hash(state);
    }
}

pub trait VocabBuilder {
    fn start(&mut self, input: String);
    fn stop(&self);
    fn progress(&self) -> f64;
    fn results(self) -> HashMap<DictEntry, usize>;
}

#[derive(Default)]
pub struct ConcurrentVocabBuilder {
    vocab_store: Arc<Mutex<HashMap<DictEntry, usize>>>,
    enabled: Arc<AtomicBool>,
    remaining_sentences: Arc<AtomicUsize>,
    total_sentences: usize,
}

impl VocabBuilder for ConcurrentVocabBuilder {
    fn start(&mut self, input: String) {
        self.enabled.store(true, Ordering::Relaxed);
        let input_vectors: Vec<String> = input
            .split(&['　', '。', '、', '「', '」', '〜', '（', '）'])
            .map(|s| s.to_string())
            .collect();

        self.total_sentences = input_vectors.len();
        // Early return in case of there not being enough vocab in List.
        if self.total_sentences < 8 {
            return;
        }

        self.remaining_sentences
            .store(input_vectors.len(), Ordering::Relaxed);
        input_vectors
            .chunks(input_vectors.len() / 8)
            .into_iter()
            .for_each(|sentences_chunk| {
                let store = self.vocab_store.clone();
                chunk_search(
                    store,
                    sentences_chunk.to_vec(),
                    self.enabled.clone(),
                    self.remaining_sentences.clone(),
                )
            });
    }

    fn stop(&self) {
        self.enabled.store(false, Ordering::Relaxed);
    }

    fn progress(&self) -> f64 {
        (1.0 - (self.remaining_sentences.load(Ordering::SeqCst) as f64
            / self.total_sentences as f64))
            * 100.0
    }
    fn results(self) -> HashMap<DictEntry, usize> {
        if let Ok(vocab_lock) = Arc::try_unwrap(self.vocab_store) {
            if let Ok(inner_map) = vocab_lock.into_inner() {
                inner_map
            } else {
                HashMap::new()
            }
        } else {
            HashMap::new()
        }
    }
}

fn chunk_search(
    store: Arc<Mutex<HashMap<DictEntry, usize>>>,
    sentences: Vec<String>,
    enabled: Arc<AtomicBool>,
    remaining: Arc<AtomicUsize>,
) {
    thread::spawn(move || {
        for sentence in sentences.into_iter() {
            let thread_id = std::thread::current().id();
            // Early return when stopped. Stopping this function will result in a closed thread.
            if !enabled.load(Ordering::Relaxed) {
                return;
            }
            let words = word_finder(&sentence);
            if let Ok(mut store_lock) = store.lock() {
                words.into_iter().for_each(|word| {
                    store_lock
                        .entry(word)
                        .and_modify(|occurence| *occurence += 1)
                        .or_insert(1);
                });
            };

            let r = remaining.fetch_sub(1, Ordering::SeqCst);
            println!("{r} on thread: {:?}", thread_id);
        }
    });
}

fn word_finder(search_str: &str) -> Vec<DictEntry> {
    let mut input: VecDeque<char> = search_str.chars().collect();
    let mut results: Vec<DictEntry> = Vec::new();

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
                results.push(DictEntry(entry));
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
        let result = word_finder("今日は何をしてるか？\n母と買い物をしていただよ");
        println!("{:?}", result)
    }

    #[test]
    fn concurrent_works() {
        let japanese_paragraph = r#"
    日本は、東アジアに位置する島国で、豊かな歴史と文化を持っています。首都は東京で、
    経済的にも世界の中心的な都市の一つです。日本には、桜の花が咲き誇る春や、紅葉が美しい秋など、四季折々の自然の美しさがあります。
    また、日本の食文化も多様で、寿司、ラーメン、天ぷらなどが世界中で人気です。さらに、アニメや漫画といったポップカルチャーも、
    日本から世界に広まっています。日本の伝統と現代が融合した魅力的な国です。
    "#.to_string();
        let mut vocab_builder = ConcurrentVocabBuilder::default();
        vocab_builder.start(japanese_paragraph);
        while vocab_builder.progress() != 100f64 {
            println!("progress: {}", vocab_builder.progress());
        }
        println!("{}", vocab_builder.results().len());
    }
}
