use lindera::tokenizer;
use lindera_core::core::viterbi::Mode;

const DETAIL_INFO_LEN:usize = 9;

#[derive(Debug, Clone)]
pub struct TokenDetailInfo {
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

impl TokenDetailInfo {
    pub fn new(detail:&[String]) -> Self {
        assert_eq!(detail.len(), DETAIL_INFO_LEN);
        TokenDetailInfo{
            part_of_speech: detail[0].clone(),
            sub_pos1: detail[1].clone(),
            sub_pos2: detail[2].clone(),
            sub_pos3: detail[3].clone(),
            conjugation_type: detail[4].clone(),
            conjugation_form: detail[5].clone(),
            base_form: detail[6].clone(),
            reading: detail[7].clone(),
            pronunciation: detail[8].clone(),
        }
    }
}

#[derive(Debug)]
pub enum TokenDetail {
    Info(TokenDetailInfo),
    Unknown,
}

impl TokenDetail {
    pub fn new(token: &tokenizer::Token) -> Self {
        if token.detail.len() == DETAIL_INFO_LEN {
            TokenDetail::Info(TokenDetailInfo::new(&token.detail))
        }else {
            Self::Unknown
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub detail: TokenDetail,
}

impl Token {
    pub fn new(word: &tokenizer::Token) -> Self {
        Token {
            text: word.text.to_string(),
            detail: TokenDetail::new(&word),
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