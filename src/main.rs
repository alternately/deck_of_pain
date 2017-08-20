extern crate deck_of_pain;

fn main() {
    println!("get ready for pain");

    use deck_of_pain::*;

    let deck = exercises::generate_exercises();
    exercises::generate_report(deck);

    

}
