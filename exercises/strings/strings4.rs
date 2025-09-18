// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// I AM DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned()); // 将引用转化为所有权
    string("nice weather".into()); // 将类型into另一个类型
    string(format!("Interpolation {}", "Station")); // 格式化到string字符串
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim()); // 产生切片
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // 转换为string
}
