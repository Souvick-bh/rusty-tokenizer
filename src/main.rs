use std::collections::HashMap;

struct  Tokenizer {
    token_to_id: HashMap<String, usize>,
    id_to_token: HashMap<usize, String>,
    token_id: usize,
}

impl Tokenizer {
    fn new() -> Self {
        Self { token_to_id: HashMap::new(), id_to_token: HashMap::new(), token_id: 1 }
    }

    // Add the special tokens, call it after initialisation
    fn add_special_tokens(&mut self) {
        self.token_to_id.insert(String::from("<UNK>"), 0);
        self.id_to_token.insert(0, String::from("<UNK>"));
    }

    fn train(&mut self, text: &str) {
        // Iterate through the text and map each word
        for word in text.split_whitespace() {
            let token = word.to_string();
            if !self.token_to_id.contains_key(&token) {
                self.token_to_id.insert(token.clone(), self.token_id);
                self.id_to_token.insert(self.token_id, token);
                self.token_id+=1;
            }
        }
    }

    fn encode(&self, text: &str) -> Vec<usize> {
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

    fn decode(&self, tokens_id_list: Vec<usize>) -> String {
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
}


fn main() {
    let text = String::from("Angena gatram nayena rajyam lavanena bhojyam prashastena cha vachanam");

    let mut tokenizer = Tokenizer::new();

    tokenizer.add_special_tokens();
    tokenizer.train(&text);

    let id_list = tokenizer.encode("Angena gatram laudena bhojyam");
    println!("{:?}", id_list);

    let tokenized_text = tokenizer.decode(id_list);
    println!("{:?}", tokenized_text);

}
