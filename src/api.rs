use crate::tokenizer::{
    Tokenizer, TokenDetail
};

pub struct KotobaPlayer {
    pub tokenizer:Tokenizer
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
            };
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
        assert_eq!(kotoba.parrot("キトさんは、とっても可愛いです。"), "可愛い! 可愛い!");
    }
}