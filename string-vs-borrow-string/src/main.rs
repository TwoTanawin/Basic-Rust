fn main() {
    // println!("Hello, world!");
    let map = String::from("Old map");
    let borrowed_map = map.as_str();
    let mut crabby_map = borrowed_map.to_string();

    crabby_map.push_str(" to new map");

    println!("Crabby map : {}", crabby_map)
}
