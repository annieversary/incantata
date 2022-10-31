use incantata::*;

fn main() {
    loop {
        // structure of the language
        let s = Structure {
            // how many characters the onset is
            onset: 1,
            // allowed characters for the onset
            onset_dict: CONSONANTS.chars().collect(),

            // how many characters the nucleus is
            nucleus: 1,
            // allowed characters for the nucleus
            nucleus_dict: VOCALS
                .chars()
                // .cycle()
                // .take(VOCALS.len() * 5)
                // .chain(VOCALS_ACCENTS.chars())
                .collect(),

            // how many characters the coda is
            coda: 0,
            // allowed characters for the coda
            coda_dict: CONSONANTS.chars().collect(),

            // minimum length of a word
            min_len: 4,
            // the words will be generated to be around this length
            // due to the way incantata works (by combining valid syllables),
            // we can't actually make a word of a given length
            suggested_len: 15,
        };

        // generate 10 words
        for _ in 0..10 {
            println!("{}", s.generate());
        }

        if std::io::stdin().read_line(&mut String::new()).is_err() {
            break;
        }
    }
}
