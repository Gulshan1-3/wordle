pub mod algorithms;

pub fn play <G:Guesser> (answer: &'static str, mut  guesser:G) -> Option<usize>{
     //play six rounds where it invokes guesser each round
     let mut history = Vec::new();

     //wordle only allows six guesses,we allow more to avoid chopping off the 
     //score distribution for the stats purposes
     for i in 1..=32{
        let guess = guesser.guess(&history);
        if guess == answer {
            return Some(i);
        }
        let correctness =  Correctness::compute(answer, &guess);
        history.push(Guess{word:guess,mask:correctness});
     }

    None
}
impl Correctness {
    fn compute(answer:&str,guess:&str) -> [Self;5]{
        assert_eq!(answer.len(),5);
        assert_eq!(answer.len(),5);
        let mut c = [Correctness::Wrong;5];
        //Mark things green 
        for (i,(a,g)) in answer.chars().zip(guess.chars()).enumerate(){
            if a == g {

                c[i] = Correctness::Correct;
            

            } 
        }

 //Mark things green 
 for (i,g) in answer.chars()
 .zip(guess.chars()).enumerate(){
    if a == g {

        //already marked green
        continue;
    

            }

            if let Some(j) = answer.chars().enumerate.find_map(|i,a)
        }
        
             todo!();
            
        }
    }


    
 

 fn check (answer: &str,guess:&str) -> [Correctness;5]{
    todo!();

 }
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
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