use incantata::*;

fn main() {
    let s = Structure {
        onset: 1,
        onset_dict: CONSONANTS.chars().collect(),
        nucleus: 1,
        nucleus_dict: VOCALS
            .chars()
            .cycle()
            .take(VOCALS.len() * 5)
            .chain(VOCALS_ACCENTS.chars())
            .collect(),
        coda: 0,
        coda_dict: CONSONANTS.chars().collect(),

        min_len: 4,
        suggested_len: 15,
    };

    for _ in 0..10 {
        println!("{}", incantata(&s));
    }
}
