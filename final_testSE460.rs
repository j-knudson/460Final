#[derive(Debug)]
struct Sentence {
    original: String,
    total_words: u32,
    characters_per_word: u32,
    total_characters: usize,
}

pub trait Counting {
    fn input(&mut self);
    fn verify_count(&mut self);

    fn output(&mut self);
}

impl Counting for Sentence {
    fn input(&mut self) {
        println!("Enter a word for use in the amazing 460 word machine");
        let mut _word = String::new();
        std::io::stdin().read_line(&mut _word).expect("Unable to read entered data");
        _word.pop();        //remove the trailing newline
        self.original = _word;

    }

    fn verify_count(&mut self)  {
        let mut split = self.original.split(' ');
        let mut counter = 0;
        for s in split {
            println!("{} has {} characters", s, s.chars().count());
            counter += 1;
            self.total_characters += s.chars().count();
        }
        self.total_words = counter;

    }


    fn output(&mut self) {
        self.characters_per_word = self.total_characters as u32/self.total_words;
        println!("The number of words provided by the user is {}. The total number of characters is {}. The characters per word is {}. ", self.total_words, self.total_characters, self.characters_per_word)
    }
}

fn main() {
    let mut word1 = Sentence {
        original: "".to_string(),
        total_words: 0,
        characters_per_word: 0,
        total_characters: 0
    };

    word1.input();
    word1.verify_count();
    word1.output();

    //println!("My sentence test: {:#?}", word1);  //test

    //println!("Original is: {}", word1.original);
}
