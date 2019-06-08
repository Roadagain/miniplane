use miniplane::creature::Creature;
use miniplane::player::Player;
fn main() {
    let p1: Player = Default::default();
    let p2: Player = Default::default();
    let c1 = Creature::new(2, 2);
    let c2 = Creature::new(3, 1);

    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", c1);
    println!("{:?}", c2);
}
