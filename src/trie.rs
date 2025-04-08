

use std::collections::HashMap;

#[derive(Default,Debug)]
pub struct TrieNode{
    pub nodes:HashMap<char, TrieNode>,
    pub is_word:bool,
}

#[derive(Debug)]
pub struct Trie{
    pub root:TrieNode,
}

impl Default for Trie {
    fn default() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }
}

pub trait Dictionary {
    fn add_word(&mut self, word: &str);
    fn is_word(&self, word: &str) -> bool;
}

impl Dictionary for Trie{
    fn add_word(&mut self, word:&str){
        let mut node = &mut self.root;
        for ch in word.chars(){
            node = node.nodes.entry(ch).or_default();
        }
        node.is_word=true;
    }

    fn is_word(&self, word:&str)->bool{
        let mut node = &self.root;
        for ch in word.chars(){
            match node.nodes.get(&ch){
                Some(nd) => node = nd,
                None => return false,
            }
        }
        node.is_word
    }
}

