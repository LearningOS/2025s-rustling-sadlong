// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    //把y改成&y 当使用 match 语句处理 y 时，y 的所有权被转移到了 match 语句块中。这就导致在 match 语句块之后，y 不能再被使用，因为它已经失去了所有权。
    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
