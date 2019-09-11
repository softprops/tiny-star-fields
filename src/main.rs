use rand::{seq::SliceRandom, Rng};
use std::iter::repeat_with;

#[rustfmt::skip]
const STARS: &[&str] = &["* ", " . ", " . ", ". ", " + ", " * ", "✵  ", "✷  ", "* ", " . ", "* ", ".", " . ", "    ", " ˚ ", " ·", " · ", "· ", " ˚ ", " · ", " ·", "·  ", ".", ". ", " ✧", " ✫ ", " ⊹ ", " *", " ⋆ ", " ✺ ", " ✹ ", " ✵ ", " ˚ ", " ✦ ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " 　", " ", "", " ", " ", " ", "　  　", " ", " ", " ", " ", " ", "　", "　", " 　", "　", " 　　", "　", "  　", "　", " 　", "　", "　　", "　", "　", "　　", "　", "　", "　 ", "　　", "　", "　　", "　　　", "　", "　", "　　", "　", "　", "　　　", "　", "　", "　　", "　", "　　　　", "　", "　　　　　", "　　 ", "　　 ", "　 ", " ", "　　　　", "  ", " ", "     ", "   ", "     ", "   ", " "];

fn stars() -> impl Iterator<Item = String> {
    let mut rng = rand::thread_rng();
    repeat_with(move || {
        let take = (rng.gen::<f32>() * 5.0).floor() as usize + 5;
        STARS
            .choose_multiple(&mut rng, take)
            .fold(String::new(), |mut res, star| {
                res.push_str(star);
                res
            })
    })
}

fn main() {
    for line in stars().take(100) {
        println!("{}", line);
    }
}
