fn open_chest(is_empty: bool) -> Option<String> {
    if is_empty {
        // Code here ..
        None
    } 
    else 
    {
        // Code here ..
        Some("You found a treasures".to_string())
    }
}

fn open_door(is_danger: bool) -> Result<String, String>{
    if is_danger {
        Err("You found a monster".to_string())
    } else {
    // Code here ..
    Ok("The door is open".to_string())
    }
}

fn main() {
    let chest_result = match open_chest(true) {
        Some(treasures) => treasures,
        None => "the chest is empty".to_string()
    };
    println!("{:?}", chest_result);
    let door_result = match open_door(false) {
        Ok(safe) => safe,
        Err(mimic) => panic!("{}", mimic)
    };
    println!("{:?}", door_result);
}