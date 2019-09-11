//! A random start field generated inspired by artwork by [@tiny_star_fields](https://twitter.com/tiny_star_field) on Twitter
use rand::{seq::SliceRandom, Rng};
use std::iter::repeat_with;

#[rustfmt::skip]
const STARS: &[&str] = &["* ", " . ", " . ", ". ", " + ", " * ", "✵  ", "✷  ", "* ", " . ", "* ", ".", " . ", "    ", " ˚ ", " ·", " · ", "· ", " ˚ ", " · ", " ·", "·  ", ".", ". ", " ✧", " ✫ ", " ⊹ ", " *", " ⋆ ", " ✺ ", " ✹ ", " ✵ ", " ˚ ", " ✦ ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " 　", " ", "", " ", " ", " ", "　  　", " ", " ", " ", " ", " ", "　", "　", " 　", "　", " 　　", "　", "  　", "　", " 　", "　", "　　", "　", "　", "　　", "　", "　", "　 ", "　　", "　", "　　", "　　　", "　", "　", "　　", "　", "　", "　　　", "　", "　", "　　", "　", "　　　　", "　", "　　　　　", "　　 ", "　　 ", "　 ", " ", "　　　　", "  ", " ", "     ", "   ", "     ", "   ", " "];

/// Return an Iterator over random ascii star field text
fn stars() -> impl Iterator<Item = String> {
    let mut rng = rand::thread_rng();
    repeat_with(move || {
        let take = (rng.gen::<f32>() * 5.0).floor() as usize + 5;
        STARS
            .choose_multiple(&mut rng, take)
            .fold(String::new(), |res, line| res + line)
    })
}

fn main() {
    for line in stars().take(100) {
        println!("{}", line);
    }
}
