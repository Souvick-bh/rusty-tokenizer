mod tokenizers;
use tokenizers::word::WordTokenizer;
use tokenizers::character::CharacterTokenizer;

fn main() {
    let text = String::from("Angena gatram nayena rajyam lavanena bhojyam prashastena cha vachanam");

    let mut tokenizer = WordTokenizer::new();
    let mut tokenizer02 = CharacterTokenizer::new();

    tokenizer.train(&text);
    tokenizer02.train(&text);

    let tokenized_text = tokenizer.tokenize_and_decode("Angena gatram laudena bhojyam");
    println!("{:?}", tokenized_text);

    let tokenized_text02 = tokenizer02.tokenize_and_decode("Angena gatram laudena bhojyam tdvgzzi");
    println!("{:?}", tokenized_text02);
}
