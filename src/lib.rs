pub const LEARNED: i8 = 5;

#[derive(Clone)]
pub struct Item {
    pub word: String,
    pub context: String,
    pub translation: String,
    pub frequency: i8,
    pub learned: i8,
}

impl Item {
    pub fn new(word: &str, translation: &str, context: &str) -> Self {
        Item {
            word: word.to_string(),
            context: context.to_string(),
            translation: translation.to_string(),
            frequency: 0,
            learned: 0,
        }
    }

    pub fn learn(&mut self) -> () {
        self.frequency = self.frequency + 1;

        if self.frequency >= LEARNED {
            if self.learned == 0 {
                let word = self.word.clone();
                self.word = self.translation.clone();
                self.translation = word;

                self.learned = 1;
                self.frequency = 0;
            } else if self.learned == 1 {
                self.learned = 2;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_word() {
        let word = "new";
        let context = "This is the `new` string";
        let translation ="новый";

        let item = Item::new(word, context, translation);

        assert_eq!(item.word, word);
        assert_eq!(item.context, context);
        assert_eq!(item.translation, translation);
        assert_eq!(item.frequency, 0);
        assert_eq!(item.learned, 0);
    }

    #[test]
    fn attempt() {
        let word = "new";
        let context = "This is the `new` string";
        let translation ="новый";

        let mut item = Item::new(word, context, translation);

        item.learn();
        assert_eq!(item.learned, 0);
        assert_eq!(item.frequency, 1);
        item.learn();
        item.learn();
        item.learn();
        item.learn();
        item.learn();
        assert_eq!(item.learned, 1);
        assert_eq!(item.word, translation);
        assert_eq!(item.translation, word);
        item.learn();
        item.learn();
        item.learn();
        item.learn();
        item.learn();
        assert_eq!(item.learned, 2);
    }
}
