fn main() {
    let x = 1;
    let y = 0.5;
    let z = x + y as i32;

    println!("Z : {}", z);

    let msg = String::from("Hello Crabby");
    let msg2 = "Hello Crabby".to_string();
    let msg3 = "Hello Crabby";
    let msg4 = format!("Point: {}, {}", x, y);
    
    println!("{}", msg4);
}
