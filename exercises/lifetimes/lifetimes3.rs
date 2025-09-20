// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

struct Book<'a> {
    author: &'a str,
    title: &'a str,
} // 引用，无法推断生命周期；如果title先被释放了则报错；标注至少与Book生命周期相同以通过编译

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
