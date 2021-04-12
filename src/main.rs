pub mod stack;
pub mod balanced_brackets;
fn main() {
    let test_1 = String::from("{()}{(([[]]))}");
    let mut game = balanced_brackets::Brackets::new(test_1);
    
    if game.start_game() {
        println!("Cadena Aceptada")
    } else {
        println!("Cadena Rechazada")
    }

}
