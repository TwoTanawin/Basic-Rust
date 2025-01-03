trait Gear {
    fn use_gear(&self);
}
struct Sword;
struct Bow;
struct Potion;
// Code here...
impl Gear for Sword {
    fn use_gear(&self) {
        println!("Swing Sword");
    }
}

impl Gear for Bow {
    fn use_gear(&self) {
        println!("Fire Arrow");
    }
}

impl Gear for Potion {
    fn use_gear(&self) {
        println!("Drink Potion")
    }
}

fn use_gear<T: Gear>(item: T){
    item.use_gear();
}

fn main() {
    let crabby_sword = Sword;
    let crabby_bow = Bow;
    let crabby_potion = Potion;
    // Code here...
    use_gear(crabby_sword);
    use_gear(crabby_bow);
    use_gear(crabby_potion);
}
