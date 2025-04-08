use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::{Trie,Dictionary};

#[derive(Default)]
pub struct TrieBuilder {
    case_insensitive: bool,
    file_path: Option<String>,
}

impl TrieBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn case_insensitive(mut self, flag: bool) -> Self {
        self.case_insensitive = flag;
        self
    }

    pub fn from_file<P: Into<String>>(mut self, path: P) -> Result<Trie, std::io::Error> {
        self.file_path = Some(path.into());
        self.build()
    }

    pub fn build(self) -> Result<Trie, std::io::Error> {
        let mut trie = Trie::default();

        if let Some(path) = self.file_path {
            let file = File::open(path)?;
            for line in BufReader::new(file).lines() {
                let word = line?.trim().to_string();
                if !word.is_empty() {
                    let word = if self.case_insensitive {
                        word.to_ascii_lowercase()
                    } else {
                        word
                    };
                    trie.add_word(&word);
                }
            }
        }

        Ok(trie)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file_check_a_word_existance() {
        let trie = TrieBuilder::new()
        .from_file("words.txt")
        .expect("Failed to load trie from file");

        let is_word = trie.is_word("test");
        assert!(is_word);
    }

    #[test]
    fn load_file_fail() {
        let result = TrieBuilder::new().from_file("does-not-exist.txt");

        assert!(
            result.is_err(),
            "Expected loading a non-existent file to return an error"
        );
    }
}