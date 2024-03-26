// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.


// 定义引用结构体时，需要为其每一个引用添加生命周期注解
// 为了确保引用结构体的实例在使用引用时是安全的，需要使用生命周期注解来表明引用结构体的字段与引用结构体实例之间的关系。
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
