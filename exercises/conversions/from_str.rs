// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
////这类似于from_into.rs，但这次我们将实现“FromStr”并//返回错误，而不是返回默认值。此外，在//实现FromStr时，可以对字符串使用“parse”方法来生成//实现者类型的对象。你可以在//https://doc.rust-lang.org/std/str/trait.FromStr.html了解更多
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

// I AM DONE

// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
//步骤:// 1。如果提供的字符串长度为0，则应该返回错误// 2。根据逗号分隔给定的字符串// 3。只有2个元素应该从split返回，否则返回// error// 4。从split操作中提取第一个元素，并将其用作名称// 5。从split操作中提取另一个元素，并将其解析为一个//` usize ` as age，类似于`" 4"。解析::< usize>()`// 6。如果在提取姓名和年龄时出错，应该返回一个错误///如果一切顺利，则返回一个Person object//
//的结果作为旁注:` box < dyn Error > ` implements ` from < & ' _ str > `。这意味着，如果//想要返回一个字符串错误消息，只需使用
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() == 0 {
            Err(Self::Err::Empty)
        } else {
            let int_v:Vec<&str> = s.split(",").collect();
            if int_v.len() != 2 {
                Err(Self::Err::BadLen)
            } else {
                match int_v.get(0) {
                    None => Err(Self::Err::BadLen),
                    Some(&Name) => {
                        if Name.len() == 0 {
                            Err(Self::Err::NoName)
                        } else {
                            match int_v.get(1) {
                                None => Err(Self::Err::BadLen),
                                Some(&age_raw) => {
                                    match age_raw.parse::<usize>() {
                                        Err(e) => Err(Self::Err::ParseInt(e)),
                                        Ok(age_) => {
                                            Ok(Person {
                                                name: Name.to_string(), // 注意是String
                                                age: age_
                                            })
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
