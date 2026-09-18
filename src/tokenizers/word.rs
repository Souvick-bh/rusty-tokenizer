use std::collections::HashMap;

pub struct  WordTokenizer {
    token_to_id: HashMap<String, usize>,
    id_to_token: HashMap<usize, String>,
    next_id: usize,
}

impl WordTokenizer {
    // Initialize a tokenizer
    pub fn new() -> Self {
        let mut tokenizer = Self { token_to_id: HashMap::new(),
             id_to_token: HashMap::new(), next_id: 1 };
        tokenizer.add_special_tokens();
        tokenizer
    }

    // Add the special tokens, call it after initialisation
    pub fn add_special_tokens(&mut self) {
        self.token_to_id.insert(String::from("<UNK>"), 0);
        self.id_to_token.insert(0, String::from("<UNK>"));
    }

    pub fn train(&mut self, text: &str) {
        // Iterate through the text and map each word
        for word in text.split_whitespace() {
            if !self.token_to_id.contains_key(word) {
                let token = word.to_string();
                self.token_to_id.insert(token.clone(), self.next_id);
                self.id_to_token.insert(self.next_id, token);
                self.next_id+=1;
            }
        }
    }

    pub fn encode(&self, text: &str) -> Vec<usize> {
        let mut tokens_id_list = Vec::new();
        // Take the input text and return vector of token id from vocabulary  
        for word in text.split_whitespace() {
            match self.token_to_id.get(word) {
                Some(id) => tokens_id_list.push(*id),
                None => tokens_id_list.push(0),
            }
        }
        tokens_id_list
    }

    pub fn decode(&self, tokens_id_list: Vec<usize>) -> String {
        let mut word_list = Vec::new();
        // Take a vector of token id as input and return the text from vocabulary table
        for id in tokens_id_list {
            match self.id_to_token.get(&id) {
                Some(word) => word_list.push(word.clone()),
                None => word_list.push("<UNK>".to_string()),
            }
        }
        word_list.join(" ")
    }

    pub fn tokenize_and_decode(&self, text: &str) ->String {
        let token_id_list = self.encode(text);
        let tokenized_text = self.decode(token_id_list);
        tokenized_text
    }
}

