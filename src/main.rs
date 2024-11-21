fn main() {

    let game_class = std::env::args().nth(1).expect("no argument given");

    println!("toto est un {}.", game_class);
}
