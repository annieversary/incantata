use rand::{thread_rng, Rng};

pub const CONSONANTS: &'static str = "bcdfghjklmnpqrstvwxyz";

pub const VOCALS: &'static str = "aeiou";
pub const VOCALS_ACCENTS: &'static str = "aeiouàèìòùáéíóúäëïöü";

// TODO replace Vec<char> with HashMap<char, f32>, where the value is the probability

pub struct Structure {
    pub onset: usize,
    pub onset_dict: Vec<char>,

    pub nucleus: usize,
    pub nucleus_dict: Vec<char>,

    pub coda: usize,
    pub coda_dict: Vec<char>,

    pub min_len: usize,
    pub suggested_len: usize,
}

pub fn incantata(structure: &Structure) -> String {
    let mut rng = thread_rng();

    let len = rng.gen_range(structure.min_len..structure.suggested_len);

    let mut s = String::new();
    while s.len() < len {
        let syl = syllable(structure);
        s.push_str(&syl);
    }

    s
}

enum State {
    Onset(usize),
    Nucleus(usize),
    Coda(usize),
}

fn syllable(structure: &Structure) -> String {
    let mut rng = thread_rng();

    let mut state = State::Onset(0);

    let mut s = String::new();
    'syl: loop {
        match state {
            State::Onset(n) => {
                if n >= structure.onset || rng.gen_bool(0.3) {
                    state = State::Nucleus(0);
                } else {
                    state = State::Onset(n + 1);
                    s.push(structure.onset_dict[rng.gen_range(0..structure.onset_dict.len())]);
                }
            }
            State::Nucleus(n) => {
                // nucleus is one or more
                if n >= structure.nucleus || (n > 0 && rng.gen_bool(0.3)) {
                    state = State::Coda(0);
                } else {
                    state = State::Nucleus(n + 1);
                    s.push(structure.nucleus_dict[rng.gen_range(0..structure.nucleus_dict.len())]);
                }
            }
            State::Coda(n) => {
                if n >= structure.coda || rng.gen_bool(0.3) {
                    break 'syl;
                } else {
                    state = State::Coda(n + 1);
                    s.push(structure.coda_dict[rng.gen_range(0..structure.coda_dict.len())]);
                }
            }
        }
    }
    s
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
            let r = incantata(&s);
            assert!(r.len() >= 4);
        }
    }
}
