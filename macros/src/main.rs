macro_rules! magic_spelling {
    // Code here…
    (fire) => {
        println!("fire");
    };
    (water) => {
        println!("water");
    };
}
fn main() {
    // Crabby Spelling
    magic_spelling!(fire);
    magic_spelling!(water);
}
