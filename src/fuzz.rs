use rand::{rngs::ThreadRng, Rng};

use crate::interface::Token;

#[derive(Debug)]
pub struct Fuzzer {
    inner: Vec<Token<'static>>,
    rng: ThreadRng,
    prob: f64,
}

const STRINGS: &[&str] = &["hello", "hi", "abc", "meow", "hehehehehe"];

impl Fuzzer {
    #[must_use]
    pub fn new(prob: f64) -> Self {
        Self {
            inner: Vec::new(),
            rng: rand::thread_rng(),
            prob,
        }
    }

    #[must_use]
    pub fn fuzz(mut self) -> Vec<Token<'static>> {
        self.fuzz_command(1.);
        self.inner
    }

    fn fuzz_command(&mut self, prob: f64) {
        self.push_rand_str();
        self.fuzz_option_list(prob);
        self.fuzz_arg_list(prob);
        self.fuzz_pipeline(prob);
    }

    fn push_rand_str(&mut self) {
        self.inner
            .push(Token::Str(STRINGS[self.rng.gen_range(0..STRINGS.len())]))
    }

    fn fuzz_option_list(&mut self, prob: f64) {
        if self.rng.gen_bool(prob) {
            self.inner.push(Token::DashDash);
            self.push_rand_str();
            self.inner.push(Token::Equals);
            self.push_rand_str();
            self.fuzz_option_list(prob * self.prob);
        }
    }

    fn fuzz_arg_list(&mut self, prob: f64) {
        if self.rng.gen_bool(prob) {
            self.push_rand_str();
            self.fuzz_arg_list(prob * self.prob)
        }
    }

    fn fuzz_pipeline(&mut self, prob: f64) {
        if self.rng.gen_bool(prob) {
            self.inner.push(Token::Pipe);
            self.fuzz_command(prob * self.prob);
        }
    }
}
