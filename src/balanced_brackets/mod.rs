use crate::stack;
use std::collections::HashMap;
pub struct Brackets {
    string: String,
    stack: stack::Stack<char>,
}

impl Brackets {
    pub fn new(s: String) -> Self {
        Self {
            string: s,
            stack: stack::Stack::new(),
        }
    }

    pub fn new_play(&mut self, s: String) {
        self.stack.clear();
        self.string = s
    }

    pub fn start_game(&mut self) -> bool{
        let mut pairs: HashMap<char, char> = HashMap::new();
        pairs.insert('}', '{');
        pairs.insert(')', '(');
        pairs.insert(']', '[');

        for c in self.string.chars() {
            match c {
                '{' | '[' | '(' => self.stack.push(c),
                '}' | ']' | ')' => {
                    let _top_char = self.stack.pop();
                    match pairs.get(&c) {
                        Some(_top_char) => (),
                        _ => return false,
                    }
                },
                _ => (),
            }
        }
        self.stack.empty()
    }
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn game_test() {
        let test_1 = String::from("()");
        let mut game = super::Brackets::new(test_1);
        assert!(game.start_game());
        
        game.new_play("(({[))".to_string());
        assert_eq!(game.start_game(), false);

        game.new_play("(([]{[]}[]))".to_string());
        assert!(game.start_game());

        game.new_play("(a([]{[99¿?]}[])abc)".to_string());
        assert!(game.start_game());
    }
}
