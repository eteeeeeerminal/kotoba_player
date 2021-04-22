use lindera::tokenizer;
use lindera_core::core::viterbi::Mode;

#[derive(Clone)]
pub struct TokenDetail {
    pub part_of_speech:String,
    pub sub_pos1:String,
    pub sub_pos2:String,
    pub sub_pos3:String,
    pub conjugation_type:String,
    pub conjugation_form:String,
    pub base_form:String,
    pub reading:String,
    pub pronunciation:String,
}

impl TokenDetail {
    pub fn new(word:tokenizer::Token) -> Self {
        TokenDetail{
            part_of_speech: word.detail[0].clone(),
            sub_pos1: word.detail[1].clone(),
            sub_pos2: word.detail[2].clone(),
            sub_pos3: word.detail[3].clone(),
            conjugation_type: word.detail[4].clone(),
            conjugation_form: word.detail[5].clone(),
            base_form: word.detail[6].clone(),
            reading: word.detail[7].clone(),
            pronunciation: word.detail[8].clone(),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub text: String,
    pub detail: TokenDetail,
}

impl Token {
    pub fn new(word: &tokenizer::Token) -> Self {
        Token {
            text: word.text.to_string(),
            detail: TokenDetail::new(word.clone()),
        }
    }
}

#[derive(Clone)]
pub struct Tokenizer {
    tokenizer:tokenizer::Tokenizer
}

impl Tokenizer {
    pub fn new(dict: &str) -> Self {
        Tokenizer {
            tokenizer: tokenizer::Tokenizer::new(Mode::Normal, dict),
        }
    }

    pub fn tokenize(&mut self, text: &str) -> Vec<Token> {
        let lindera_tokens = self.tokenizer.tokenize(text);
        let mut tokens = Vec::new();
        for token in lindera_tokens {
            tokens.push(Token::new(&token));
        }
        tokens
    }
}