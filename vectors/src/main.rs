fn main() {
    // println!("Hello, world!");
    let mut items = vec!["Gold", "Silver", "Ruby", "Emerald"];
    items.push("Diamond");
    items.remove(1);
    println!("Items : {:?}",items);
    println!("Items Lenght : {:?}",items.len());
    println!("Items Capacity : {:?}",items.capacity());
}
