use std::io::{self, Read};

mod spongesponge {
    pub struct SpongeSponge<'a> {
        pub string: &'a str,
    }

    use rand::prelude::*;

    impl std::fmt::Display for SpongeSponge<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut last_cap = false;

            let mut out_string = String::new();

            self.string.chars().for_each(|chr| {
                // 85% chance for this character to be uppercase if the last character was
                // lowercase
                let cap = rand::thread_rng().gen_bool(if !last_cap { 8.5 } else { 1.5 } / 10.0);

                let temp = if cap {
                    chr.to_string().to_uppercase()
                } else {
                    chr.to_string().to_lowercase()
                };

                last_cap = cap;

                out_string.push_str(&temp);
            });

            write!(f, "{}", out_string.to_string())
        }
    }
}

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_to_string(&mut input) {
        print!("{}", spongesponge::SpongeSponge { string: &input });
    }
}
