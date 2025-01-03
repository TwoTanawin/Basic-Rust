fn main() {
    println!("Hello, world!");
    println!("------IF ELSE------");
    decision();
    println!("------MATCH------");
    enemyList();
    println!("------Loop------");
    repeatedTask();
    println!("------Loop------");
    repeatedTask2();
}

fn decision (){
    let weather = "rainy";
    if weather == "sunny"{
        println!("Crabby will cross the river by swimming!")
    }
    else if weather == "rainy" {
        println!("Crabby will build a bridge to stay dry.")
    }
    else {
        println!("Crabby will wait for better weather.")
    }
}

fn enemyList(){
    let enemy = "troll";
    match enemy {
        "goblin" => print!("he uses his rusty sword to attack."),
        "troll" => println!("Crabby sets a trap!"),
        "dragon" => println!("Crabby runs for cover!"),
        _ => println!("Crabby is confused...")
    }
}

fn repeatedTask(){
    let mut wood = 0;
    while wood < 10{
        println!("Wood : {}", wood);

        wood += 1;
    }

    print!("Total Wood : {}", wood)
}

fn repeatedTask2(){
    let mut wood = 0;
    loop {
        wood += 1;
        println!("Wood : {}", wood);
        if wood == 10{
            print!("Finish Corrected Wood : {}", wood);
            break;
        }
    }
}
