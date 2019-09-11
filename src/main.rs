use rand::seq::SliceRandom;
use rand::Rng;
use std::iter::repeat_with;

fn stars<R>(rng: &mut R) -> String
where
    R: Rng,
{
    #[rustfmt::skip]
    let mut stars = vec!["* ", " . ", " . ", ". ", " + ", " * ", "✵  ", "✷  ", "* ", " . ", "* ", ".", " . ", "    ", " ˚ ", " ·", " · ", "· ", " ˚ ", " · ", " ·", "·  ", ".", ". ", " ✧", " ✫ ", " ⊹ ", " *", " ⋆ ", " ✺ ", " ✹ ", " ✵ ", " ˚ ", " ✦ ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " 　", " ", "", " ", " ", " ", "　  　", " ", " ", " ", " ", " ", "　", "　", " 　", "　", " 　　", "　", "  　", "　", " 　", "　", "　　", "　", "　", "　　", "　", "　", "　 ", "　　", "　", "　　", "　　　", "　", "　", "　　", "　", "　", "　　　", "　", "　", "　　", "　", "　　　　", "　", "　　　　　", "　　 ", "　　 ", "　 ", " ", "　　　　", "  ", " ", "     ", "   ", "     ", "   ", " "];
    stars.shuffle(rng);
    let take = (rng.gen::<f32>() * 5.0).floor() as usize + 5;
    stars[..take].join("")
}

fn main() {
    let mut rng = rand::thread_rng();
    for line in repeat_with(|| stars(&mut rng)).take(100) {
        println!("{}", line);
    }
}
