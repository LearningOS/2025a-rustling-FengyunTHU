// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//From特征用于值到值的转换。如果From为一个类型正确地实现了//，那么Into特征应该反过来工作。你可以在https://doc.rust-lang.org/std/convert/trait.From.html了解更多
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results
//您的任务是完成这个实现，以便编译“let p =// Person::from("Mark，20 ")”行。请注意，您需要将// age组件解析为类似于“4”的“usize”。解析::< usize >()`。//这种结果需要妥善处理。////步骤:// 1。如果提供的字符串长度为0，则返回默认值// Person。// 2.根据逗号分割给定的字符串。// 3.从split操作中提取第一个元素，并将其用作名称。// 4.如果姓名为空，则返回缺省的Person。// 5.从split操作中提取另一个元素，并将其解析为// `usize '作为年龄。//如果解析年龄时出错，则返回默认值//Person；否则，返回实例化的Person对象和结果
// I AM DONE

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.len() == 0 {
            Person::default()
        } else {
            let int_v:Vec<&str> = s.split(",").collect();
            // let Name:&str = &int_v[0];
            if int_v.len() >= 3 {
                Person::default()
            } else {
                match int_v.get(0) {
                    None => Person::default(),
                    Some(&Name) => { // 这里内部为&str，使用match解引用后得到Option<&T>即Option<&&str>，需要用Some(&T)匹配出&str
                        if Name.len() == 0 {
                            Person::default()
                        } else {
                            // let mut age_raw:&str = &int_v[1];
                            match int_v.get(1) {
                                None => Person::default(),
                                Some(&age_raw) => {
                                        match age_raw.parse::<usize>() {
                                        Err(e) => Person::default(),
                                        Ok(age_) => Person {
                                            name: String::from(Name),
                                            age: age_
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
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
