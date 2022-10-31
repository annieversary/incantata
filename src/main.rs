use clap::*;
use incantata::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value_t = 1)]
    onset: usize,
    #[clap(short, long, default_value_t = 1)]
    nucleus: usize,
    #[clap(short, long, default_value_t = 0)]
    coda: usize,
    /// minimum length of a word
    #[clap(short, long, default_value_t = 4)]
    min_len: usize,
    #[clap(short, long, default_value_t = 15)]
    suggested_len: usize,
}

fn main() {
    let cli = Cli::parse();

    loop {
        // structure of the language
        let s = Structure {
            // how many characters the onset is
            onset: cli.onset,
            // allowed characters for the onset
            onset_dict: CONSONANTS.chars().collect(),

            // how many characters the nucleus is
            nucleus: cli.nucleus,
            // allowed characters for the nucleus
            nucleus_dict: VOCALS.chars().collect(),

            // how many characters the coda is
            coda: cli.coda,
            // allowed characters for the coda
            coda_dict: CONSONANTS.chars().collect(),

            // minimum length of a word
            min_len: cli.min_len,
            // the words will be generated to be around this length
            // due to the way incantata works (by combining valid syllables),
            // we can't actually make a word of a given length
            suggested_len: cli.suggested_len,
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
