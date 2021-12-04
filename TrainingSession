#[derive(Debug)]
struct Word {
    original: String,
    backwards: String,
}

pub trait Palindrome {
    fn input(&mut self);
    fn verify(&self) -> bool;
    fn reverse(s: &str) -> String;
    fn output(&self);
}

impl Palindrome for Word {
    fn input(&mut self) {
        println!("Enter a word for use in the amazing 460 Palindromizer");
        let mut _word = String::new();
        std::io::stdin().read_line(&mut _word).expect("Unable to read entered data");
        //println!("The word you entered is {}", _word);
        _word.pop();        //remove the trailing newline
        self.backwards = Self::reverse(&_word);
        self.original = _word;

    }

    fn verify(&self) -> bool {
        let verification_result = self.original.eq(&self.backwards);
        verification_result
    }

    fn reverse(s: &str) -> String {
       s.chars().rev().collect()
    }

    fn output(&self) {
        if self.verify() {
            println!("The Great Palindromizer has determined that your word {} is a palindrome", self.original)
        }
        else {
            println!("The Great Palindromizer has determined that your word {} is NOT a palindrome", self.original)
        }
    }
}

fn main() {
    let mut word1 = Word {
        original: "".to_string(),
        backwards: "".to_string(),
    };

    /*
    println!("My word test: {:#?}", word1);  //test
    println!("Original is: {}", word1.original);
    println!("Backwards is: {}", word1.backwards);
     */
    word1.input();

    /*
    println!("My word test: {:#?}", word1);  //test
    println!("Original is: {}", word1.original);
    println!("Backwards is: {}", word1.backwards);
    println!("Palindrome test is: {}", word1.verify());
     */
    word1.output();
}
