pub mod utils {

    pub fn run<TIn, TOut>(input: TIn, func: fn(TIn) -> TOut, ans: Option<TOut>) 
    where TIn: std::fmt::Debug + std::cmp::PartialEq,
          TOut: std::fmt::Debug + std::cmp::PartialEq
    {
        let answer = func(input);

        if let Some(correct) = ans {
            if answer == correct {
                println!("Correct!");
            } else {
                println!("Incorrect! Correct answer: {:?}", correct);
            }
        } else {
            println!("Answer: {:?}", answer);
        }
    }

    pub fn take_char(input: &str, c: char) -> Option<(char, &str)> {
        if let Some(ch) = input.chars().next() {
            if ch == c {
                return Some((ch, &input[1..]));
            }
        }
        None
    }

    pub fn take_whitespace(input: &str) -> Option<(&str, &str)> {
        take_while1(input, |c| c.is_whitespace())
    }

    pub fn take_word<'a>(input: &'a str, word: &'static str) -> Option<(&'a str, &'a str)> {
        if input.starts_with(word) {
            Some((word, &input[word.len()..]))
        } else {
            None
        }
    }

    pub fn take_list(input: &str, parser: fn(&str) -> Option<(&str, &str)>, delimiter: fn(&str) -> Option<(&str, &str)>) -> Option<(Vec<&str>, &str)> {
        let mut result = Vec::new();
        let mut input = input;
        while let Some((item, rest)) = parser(input) {
            result.push(item);
            input = rest;
            if let Some((_, rest)) = delimiter(input) {
                input = rest;
            } else {
                break;
            }
        }
        if result.is_empty() {
            None
        } else {
            Some(( result, input ))
        }
    }

    pub fn take_tuple(input: &str, parsers: (fn(&str) -> Option<(&str, &str)>, fn(&str) -> Option<(&str, &str)>)) -> Option<(&str, &str)> {
        let (parser1, parser2) = parsers;
        if let Some((item1, rest)) = parser1(input) {
            if let Some((item2, rest)) = parser2(rest) {
                return Some((&input[..(item1.len() + item2.len())], rest));
            }
        }
        None
    }

    pub fn take_maybe(input: &str, parser: fn(&str) -> Option<(&str, &str)>) -> Option<(&str, &str)> {
        if let Some((item, rest)) = parser(input) {
            Some((item, rest))
        } else {
            Some(("", input))
        }
    }

    pub fn take_many(input: &str, func: fn(&str) -> Option<(&str, &str)>) -> Option<Vec<&str>> {
        let mut result = Vec::new();
        let mut input = input;
        while let Some((item, rest)) = func(input) {
            result.push(item);
            input = rest;
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn take_number(input: &str) -> Option<(&str, &str)> {
        take_while1(input, |c| c.is_digit(10))
    }

    pub fn take_while1(input: &str, predicate: fn(char) -> bool) -> Option<(&str, &str)> {
        if let Some(ch) = input.chars().next() {
            if predicate(ch) {
                let mut len = 1;
                for ch in input[1..].chars() {
                    if predicate(ch) {
                        len += 1;
                    } else {
                        break;
                    }
                }
                return Some((&input[..len], &input[len..]));
            }
        }
        None

    }
}
