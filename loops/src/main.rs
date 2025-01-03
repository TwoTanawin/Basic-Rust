fn main() {
    let treasures = ["Gold", "Silver", "Ruby Gem", "Emerald"];
    let mut energy = 5;
    // Your code here …
    for treasure in treasures.iter(){
        if energy == 0{
            println!("Game Over!!");
            break;
        }else if treasure == &"Ruby Gem" {
            println!("You Win!");
            break;
        }

        energy =- 1;

        println!("Enery : {}", energy);
    }
}
