//! A random start field inspired by artwork by [@tiny_star_fields](https://twitter.com/tiny_star_field) on Twitter
use rand::{seq::SliceRandom, Rng};
use std::iter::repeat_with;
use structopt::StructOpt;

#[rustfmt::skip]
const STARS: &[&str] = &["* ", " . ", " . ", ". ", " + ", " * ", "✵  ", "✷  ", "* ", " . ", "* ", ".", " . ", "    ", " ˚ ", " ·", " · ", "· ", " ˚ ", " · ", " ·", "·  ", ".", ". ", " ✧", " ✫ ", " ⊹ ", " *", " ⋆ ", " ✺ ", " ✹ ", " ✵ ", " ˚ ", " ✦ ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " 　", " ", "", " ", " ", " ", "　  　", " ", " ", " ", " ", " ", "　", "　", " 　", "　", " 　　", "　", "  　", "　", " 　", "　", "　　", "　", "　", "　　", "　", "　", "　 ", "　　", "　", "　　", "　　　", "　", "　", "　　", "　", "　", "　　　", "　", "　", "　　", "　", "　　　　", "　", "　　　　　", "　　 ", "　　 ", "　 ", " ", "　　　　", "  ", " ", "     ", "   ", "     ", "   ", " "];

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(default_value="5", long, short)]
    columns: usize,
}


/// Return an Iterator over random ascii star field text
fn stars(options: Options) -> impl Iterator<Item = String> {
    let Options { columns } = options;
    let mut rng = rand::thread_rng();
    repeat_with(move || {
        let take = (rng.gen::<f32>() * columns as f32).floor() as usize + columns;
        STARS
            .choose_multiple(&mut rng, take)
            .fold(String::new(), |res, line| res + line)
    })
}

fn main() {
    let options = Options::from_args();
    for line in stars(options).take(100) {
        println!("{}", line);
    }
}
