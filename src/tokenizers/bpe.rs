use std::{collections::HashMap, process::ExitCode};

pub struct BPETokenizer {
    vocab: HashMap<String, usize>,
    merges: Vec<(String, String)>,
    id_to_token: HashMap<usize, String>,
    next_id: usize
}

impl BPETokenizer {

    fn initialize_words(&self, text: &str) ->Vec<Vec<String>> {
        let mut words: Vec<Vec<String>> = Vec::new();
        for word in text.split_whitespace() {
            let mut chars: Vec<String> = word.chars().map(|c| c.to_string()).collect();
            chars.push("</w>".to_string());
            words.push(chars);
        }
        words
    }

    fn get_stats(&self, words: &Vec<Vec<String>>) -> HashMap<(String, String), usize> {
        let mut stats: HashMap<(String, String), usize> = HashMap::new();
        for word in words {
            if word.len() < 2 {
                continue;
            }
            for i in 0..word.len()-1 {
                let pair = (word[i].clone(), word[i+1].clone());
                let count = stats.entry(pair).or_insert(0);
                *count+=1;
            }
        }
        stats
    }

    fn get_best_pair(&self, stats: &HashMap<(String, String), usize>) -> Option<(String, String)> {
        let mut best_pair: Option<(String, String)> = None;
        let mut max_count = 0;

        for (pair, count) in stats {
            if *count > max_count {
                max_count = *count;
                best_pair = Some(pair.clone());
            }
        }
        best_pair
    }

    fn merge_pair(&self, words: &mut Vec<Vec<String>>,pair: &(String, String)) {
        for word in words.iter_mut() {
            let mut new_word: Vec<String> = Vec::new();
            let mut i = 0;
            while i < word.len() {
                if i+1 < word.len() && word[i] == pair.0 && word[i+1] == pair.1 {
                    let merged = format!("{}{}", word[i], word[i+1]);
                    new_word.push(merged);
                    i+=2;
                } else {
                    new_word.push(word[i].clone());
                    i+=1;
                }
            }
            *word = new_word
        }
    }

    fn add_token(&mut self, token: String) {
        if !self.vocab.contains_key(&token) {
            self.vocab.insert(token.clone(), self.next_id);
            self.id_to_token.insert(self.next_id, token);
            self.next_id+=1;
        }
    }

    fn apply_merges(&self, mut tokens: Vec<String>) -> Vec<String> {
        for merge in &self.merges {
            let mut new_tokens: Vec<String> = Vec::new();
            let mut i = 0;
            while i < tokens.len() {
                if i+1 < tokens.len() && tokens[i] == merge.0 && tokens[i+1] == merge.1 {
                    let merged = format!("{}{}", tokens[i], tokens[i+1]);
                    new_tokens.push(merged);
                    i+=2;
                } else {
                    new_tokens.push(tokens[i].clone());
                    i+=1;
                }
            }
            tokens = new_tokens;
        }
        tokens
    }

    pub fn new() -> Self {
        let mut tokenizer = Self {
            vocab: HashMap::new(),
            merges: Vec::new(),
            id_to_token: HashMap::new(),
            next_id: 1
        };
        tokenizer.add_special_tokens();
        tokenizer
    }

    pub fn add_special_tokens(&mut self) {
        self.vocab.insert("<UNK>".to_string(), 0);
        self.id_to_token.insert(0, "<UNK>".to_string());
    }

    pub fn train(&mut self, text: &str, target_vocab_size: usize) {
        let mut words = self.initialize_words(text);
        for word in &words {
            for token in word {
                self.add_token(token.clone());
            }
        }
        while self.vocab.len() < target_vocab_size {
            let stats = self.get_stats(&words);
            if stats.is_empty() {
                break;
            }
            let best_pair = match self.get_best_pair(&stats) {
                Some(pair) => pair,
                None => break
            };
            self.merge_pair(&mut words, &best_pair);
            let merged = format!("{}{}", best_pair.0, best_pair.1);
            self.add_token(merged);
            self.merges.push(best_pair);
        }
    }

    pub fn encode(&self, text: &str) -> Vec<usize> {
        let mut encoded_list: Vec<usize> = Vec::new();
        for word in text.split_whitespace() {
            let mut tokens: Vec<String> = word.chars().map(|c| c.to_string()).collect();
            tokens.push("</w>".to_string());
            let tokens = self.apply_merges(tokens);
            for token in tokens {
                match self.vocab.get(&token) {
                    Some(id) => encoded_list.push(*id),
                    None => encoded_list.push(0),
                }
            }
        }
        encoded_list
    }

    pub fn decode(&self, encoded_list: Vec<usize>) -> String {
        let mut decoded_result = String::new();
        for id in encoded_list {
            if let Some(token) = self.id_to_token.get(&id) {
                if token.ends_with("</w>") {
                    let word = token.replace("</w>", " ");
                    decoded_result.push_str(&word);
                } else {
                    decoded_result.push_str(token);
                }
            } else {
                decoded_result.push_str("<UNK>")
            }
        }
        decoded_result.trim().to_string()
    }

    pub fn encode_decode(&self, text: &str) -> String {
        let encoded_list = self.encode(text);
        let decoded_result = self.decode(encoded_list);
        decoded_result
    }

}