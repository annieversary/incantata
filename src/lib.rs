use rand::{thread_rng, Rng};

pub const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";

pub const VOCALS: &str = "aeiou";
pub const VOCALS_ACCENTS: &str = "aeiouàèìòùáéíóúäëïöü";

/// structure of the language
pub struct Structure {
    /// how many characters the onset is
    pub onset: usize,
    /// allowed characters for the onset
    pub onset_dict: Vec<char>,

    /// how many characters the nucleus is
    pub nucleus: usize,
    /// allowed characters for the nucleus
    pub nucleus_dict: Vec<char>,

    /// how many characters the coda is
    pub coda: usize,
    /// allowed characters for the coda
    pub coda_dict: Vec<char>,

    /// minimum length of a word
    pub min_len: usize,
    // the words will try to be close to `suggested_len`
    //
    // due to the way incantata works (by combining valid syllables),
    // it can't actually make a word of a given length
    pub suggested_len: usize,
}

impl Default for Structure {
    fn default() -> Self {
        Self {
            onset: 1,
            onset_dict: CONSONANTS.chars().collect(),
            nucleus: 1,
            nucleus_dict: VOCALS.chars().collect(),
            coda: 0,
            coda_dict: Default::default(),
            min_len: 4,
            suggested_len: 5,
        }
    }
}

impl Structure {
    pub fn generate(&self) -> String {
        assert!(
            self.onset > 0 || self.nucleus > 0 || self.coda > 0,
            "at least one should be non-zero"
        );

        let mut rng = thread_rng();

        let len = rng.gen_range(self.min_len..self.suggested_len);

        let mut s = String::new();
        while s.len() < len {
            let syl = self.syllable();
            s.push_str(&syl);
        }

        s
    }
    fn syllable(&self) -> String {
        let mut rng = thread_rng();

        let mut state = State::Onset(0);

        let mut s = String::new();
        'syl: loop {
            match state {
                State::Onset(n) => {
                    if n >= self.onset || rng.gen_bool(0.3) {
                        state = State::Nucleus(0);
                    } else {
                        state = State::Onset(n + 1);
                        s.push(self.onset_dict[rng.gen_range(0..self.onset_dict.len())]);
                    }
                }
                State::Nucleus(n) => {
                    // nucleus is one or more
                    if n >= self.nucleus || (n > 0 && rng.gen_bool(0.3)) {
                        state = State::Coda(0);
                    } else {
                        state = State::Nucleus(n + 1);
                        s.push(self.nucleus_dict[rng.gen_range(0..self.nucleus_dict.len())]);
                    }
                }
                State::Coda(n) => {
                    if n >= self.coda || rng.gen_bool(0.3) {
                        break 'syl;
                    } else {
                        state = State::Coda(n + 1);
                        s.push(self.coda_dict[rng.gen_range(0..self.coda_dict.len())]);
                    }
                }
            }
        }
        s
    }
}

enum State {
    Onset(usize),
    Nucleus(usize),
    Coda(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = Structure {
            onset: 1,
            onset_dict: CONSONANTS.chars().collect(),
            nucleus: 1,
            nucleus_dict: VOCALS.chars().collect(),
            coda: 1,
            coda_dict: CONSONANTS.chars().collect(),
            min_len: 4,
            suggested_len: 10,
        };

        for _ in 0..100 {
            let r = s.generate();
            assert!(r.len() >= 4);
        }
    }

    #[test]
    fn default_works() {
        let s = Structure::default();

        for _ in 0..100 {
            let r = s.generate();
            assert!(r.len() >= 4);
        }
    }

    #[test]
    #[should_panic]
    fn empty_panics() {
        let s = Structure {
            onset: 0,
            onset_dict: CONSONANTS.chars().collect(),
            nucleus: 0,
            nucleus_dict: VOCALS.chars().collect(),
            coda: 0,
            coda_dict: CONSONANTS.chars().collect(),
            min_len: 4,
            suggested_len: 10,
        };

        for _ in 0..100 {
            let r = s.generate();
            assert!(r.len() >= 4);
        }
    }
}
