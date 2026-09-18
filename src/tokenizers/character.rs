use std::collections::HashMap;

pub struct CharacterTokenizer {
    token_to_id: HashMap<String, usize>,
    id_to_token: HashMap<usize, String>,
    next_id: usize
}

impl CharacterTokenizer {

    pub fn new() -> Self{
        let mut tokenizer = Self {token_to_id: HashMap::new(),
            id_to_token: HashMap::new(), next_id: 1};
        tokenizer.add_special_tokens();
        tokenizer
    }

    pub fn add_special_tokens(&mut self) {
        self.token_to_id.insert(String::from("<UNK>"), 0);
        self.id_to_token.insert(0,String::from("<UNK>"));
    }

    pub fn train(&mut self, text: &str) {
        for ch in text.chars() {
            if !self.token_to_id.contains_key(&ch.to_string()) {
                let token = ch.clone();
                self.token_to_id.insert(token.clone().to_string(), self.next_id);
                self.id_to_token.insert(self.next_id, token.to_string());
                self.next_id+=1;
            }
        }
    }

    pub fn encode(&mut self, text: &str) ->Vec<usize> {
        let mut token_id_vec: Vec<usize> = Vec::new();
        for ch in text.chars() {
            match self.token_to_id.get(&ch.to_string()) {
                Some(id) => token_id_vec.push(*id),
                None => token_id_vec.push(0),
            }
        }
        token_id_vec
    }

    pub fn decode(&mut self, token_id_vec: Vec<usize>) -> String {
        let mut char_list = Vec::new();
        for id in token_id_vec {
            match self.id_to_token.get(&id) {
                Some(ch) => char_list.push(ch.clone()),
                None => char_list.push("<UNK>".to_string())
            }
        }
        char_list.join("")
    }

    pub fn tokenize_and_decode(&mut self, text: &str) ->String {
        let token_id_vec = self.encode(text);
        let char_list = self.decode(token_id_vec);
        char_list
    }
}