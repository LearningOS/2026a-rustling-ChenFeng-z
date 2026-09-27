use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

#[derive(Debug, PartialEq)]
enum ParsePersonError {
    Empty,
    BadLen,
    NoName,
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        let fields: Vec<&str> = s.split(',').collect();

        if fields.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let name = fields[0];
        let age_str = fields[1];

        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        let age = age_str
            .parse::<usize>()
            .map_err(ParsePersonError::ParseInt)?;

        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let person = "Mark,20".parse::<Person>();
    println!("{person:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(Empty));
    }

    #[test]
    fn good_input() {
        let person = "John,32".parse::<Person>().unwrap();
        assert_eq!(person.name, "John");
        assert_eq!(person.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(NoName) | Err(ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName) | Err(ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}