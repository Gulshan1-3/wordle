pub mod algorithms;

pub fn play <G:Guesser> (answer: &'static str, guesser:G) {
     //play six rounds where it invokes guesser each round
}

 pub enum Correctness {
    //green
    Correct,
    ///yellow
    Misplaced,
    ///Gray
    Wrong,
}

pub struct Guess {
    word: String,
    mask:[Correctness;5],
}
 
 



pub trait Guesser {
    fn guess(&mut self,history: &[Guess]) -> String;
}