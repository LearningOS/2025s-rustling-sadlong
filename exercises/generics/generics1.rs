// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    //&str是字符串切片 是一个借用类型 如果要用String的话就要将milk改成String::from("milk")
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
