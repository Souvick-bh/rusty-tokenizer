mod tokenizers;
use tokenizers::word::WordTokenizer;
use tokenizers::character::CharacterTokenizer;
use tokenizers::bpe::BPETokenizer;

fn main() {
    let text = String::from("Angena gatram nayena rajyam lavanena bhojyam prashastena cha vachanam");

    let mut tokenizer = WordTokenizer::new();
    let mut tokenizer02 = CharacterTokenizer::new();
    let mut tokenizer03 = BPETokenizer::new();

    tokenizer.train(&text);
    tokenizer02.train(&text);
    tokenizer03.train(&text, 50);

    let test_text = "Angena gatram laudena bhojyam tdvgzzi";

    let sequence_length = tokenizer.encode(&test_text);
    println!("{:?}", sequence_length.len());
    let tokenized_text = tokenizer.tokenize_and_decode(test_text);
    println!("{:?}", tokenized_text);

    let sequence_length02 = tokenizer02.encode(&test_text);
    println!("{:?}", sequence_length02.len());
    let tokenized_text02 = tokenizer02.tokenize_and_decode(test_text);
    println!("{:?}", tokenized_text02);

    let sequence_length03 = tokenizer03.encode(&test_text);
    println!("{:?}", sequence_length03.len());
    let tokenized_text03 = tokenizer03.encode_decode(test_text);
    println!("{:?}", tokenized_text03);
}
