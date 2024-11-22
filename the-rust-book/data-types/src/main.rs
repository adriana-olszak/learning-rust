fn main() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;
}
