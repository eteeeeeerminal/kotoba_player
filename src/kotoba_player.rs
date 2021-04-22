use crate::tokenizer::Tokenizer;

pub struct KotobaPlayer {
    tokenizer:Tokenizer
}

impl KotobaPlayer {
    pub fn new(dict: &str) -> Self {
        KotobaPlayer {
            tokenizer: Tokenizer::new(dict),
        }
    }

    pub fn parrot(&mut self, text: &str) -> String {
        // 受け取ったテキストから、オウムのセリフを生成します。
        let tokens = self.tokenizer.tokenize(text);

        let mut parrot_word = "";
        for token in tokens.iter().rev() {
            parrot_word = match token.detail.part_of_speech.as_str() {
                "名詞" | "動詞" | "形容詞" => token.text.as_str(),
                _ => "",
            };
            if parrot_word.len() > 0 {
                break;
            }
        }

        format!("{}! {}!", parrot_word, parrot_word)
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
    }
}