use once_cell::sync::Lazy;
use tiktoken_rs::{cl100k_base, CoreBPE};

static TOKENIZER: Lazy<CoreBPE> = Lazy::new(|| cl100k_base().expect("Failed to load tokenizer"));

pub fn preload_tokenizer() {
    let _ = &*TOKENIZER;
}

pub fn estimate_tokens(text: &str) -> usize {
    let tokens = TOKENIZER.encode_with_special_tokens(text);
    tokens.len()
}
