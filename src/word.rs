use crate::DICT;

/// A word is this u16
pub struct Word(u16);

impl Word {
    /// Return a string representation of the word
    pub fn str(&self) -> &'static str {
        let bytes = DICT[self.0 as usize];
        std::str::from_utf8(&bytes).unwrap()
    }

    pub fn compare(&self, other: &Word) -> bool {

    }
}
