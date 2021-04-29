use crate::tokenizer::{
    Tokenizer, TokenDetail
};

pub struct KotobaPlayer {
    pub tokenizer:Tokenizer
}

pub enum Mask {
    WordByWord(String),
    CharByChar(char)
}

impl KotobaPlayer {
    pub fn new(dict: &str) -> Self {
        KotobaPlayer {
            tokenizer: Tokenizer::new(dict),
        }
    }

    /// 受け取ったテキストから、オウムっぽいテキストを生成します。
    pub fn parrot(&mut self, text: &str) -> String {
        let tokens = self.tokenizer.tokenize(text);
        let mut parrot_word = "";
        for token in tokens.iter().rev() {
            let detail = match &token.detail {
                TokenDetail::Info(d) => d,
                TokenDetail::Unknown => continue,
            };
            if let "名詞" | "動詞" | "形容詞" = detail.part_of_speech.as_str() {
                parrot_word = token.text.as_str();
                break;
            }
        }

        format!("{}! {}!", parrot_word, parrot_word)
    }

    /// 受け取ったテキストの名詞を伏せます。
    pub fn masquerade(&mut self, text: &str, mask: Mask) -> String {
        let tokens = self.tokenizer.tokenize(text);
        let mut masked_text = String::new();
        for token in tokens.iter() {
            let detail = match &token.detail {
                TokenDetail::Info(d) => d,
                TokenDetail::Unknown => {
                    masked_text.push_str(
                        &KotobaPlayer::mask_word(&token.text, &mask)
                    );
                    continue
                }
            };
            if let "名詞" = detail.part_of_speech.as_str() {
                masked_text.push_str(
                    &KotobaPlayer::mask_word(&token.text, &mask)
                );
            } else {
                masked_text.push_str(&token.text);
            }
        }
        masked_text
    }

    /// マスクされた単語を返します。
    fn mask_word(word: &str, mask: &Mask) -> String {
        match mask {
            Mask::CharByChar(c) => {
                let n = word.chars().count();
                let mut masked_word = String::new();
                for _ in 0..n {
                    masked_word.push(*c);
                }
                masked_word
            }
            Mask::WordByWord(ref w) => w.to_string()
        }
    }
}

#[cfg(test)]
mod kotoba_tests {
    use super::*;

    const TEST_DIC_PATH:&str = "/app/dic";
    #[test]
    fn test_parrot() {
        // 他にもテストケース書く
        let mut kotoba = KotobaPlayer::new(TEST_DIC_PATH);
        assert_eq!(kotoba.parrot("お宝はいただくぜ"), "いただく! いただく!");
        assert_eq!(kotoba.parrot("キトさんは、とっても可愛いです。"), "可愛い! 可愛い!");
    }

    #[test]
    fn test_masquerade() {
        let mut kotoba = KotobaPlayer::new(TEST_DIC_PATH);
        // やっぱり未知語がくると、tokenizeきつい
        assert_eq!(kotoba.masquerade("珠響そうきはVtuberです。", Mask::CharByChar('😺')), "😺😺😺😺😺😺😺😺😺😺😺😺です。");
        assert_eq!(kotoba.masquerade("珠響そうきはVtuberです。", Mask::WordByWord("ほげ".to_string())), "ほげほげほげほげほげです。");
    }
}