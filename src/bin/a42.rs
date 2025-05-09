// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

struct Score{
    scores: isize,
    powerup: isize
}
impl Score {
    fn new(scores: isize, powerup: isize) -> Self {
        Self { scores, powerup }
    }
}
impl Iterator for Score {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.scores += self.powerup;
        if self.powerup >= 0{
            Some(self.scores)
        } else {
            None
        }
        
    }
    
}

fn main() {
    let mut score = Score::new(0, 10);
    println!("{:?}", score.next());
    println!("{:?}", score.next());
    println!("{:?}", score.next());
    println!("{:?}", score.next());
    println!("{:?}", score.next());
    println!("{:?}", score.next());
    println!("{:?}", score.next());
}
