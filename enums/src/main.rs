enum CrabbyState {
    Fighting,
    Collecting(u32),
    Defending,
}

impl CrabbyState {
    fn state_repesent(&self){
        match self {
            CrabbyState::Fighting => println!("Crabby is fighting"),
            CrabbyState::Collecting(amount)=>println!("Crabby is collecting : {}", amount),
            CrabbyState::Defending => println!("Crabby is defending"),
        }
    }
}

fn main() {
    // println!("Hello, world!");
    let fighting = CrabbyState::Fighting;
    let collecting = CrabbyState::Collecting(20);
    let defending = CrabbyState::Defending;

    fighting.state_repesent();
    collecting.state_repesent();
    defending.state_repesent();
}
