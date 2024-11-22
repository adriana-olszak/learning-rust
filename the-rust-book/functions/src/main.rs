fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is: {x}");
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}.");
}
fn five() -> i32 {
    5
}
