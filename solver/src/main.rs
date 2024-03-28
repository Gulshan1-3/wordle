const GAMES:&str = include_str!("../answers.txt");

fn main() {
let guesser = solver::algorithms::Naive::new();
for answer in GAMES.split_whitespace(){
solver::play(answer, guesser);
}
   
}

