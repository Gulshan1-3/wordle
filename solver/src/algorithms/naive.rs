use crate::{Guesser,Guess,Correctness};

pub struct Naive;

impl Guesser for Naive {
    fn guess (&mut self,history:&[Guess]) -> String{
        todo!();
    }
}