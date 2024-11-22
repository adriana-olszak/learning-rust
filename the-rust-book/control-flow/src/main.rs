fn main() {
    // To practice with the concepts discussed in this chapter, try building programs to do the following:

    // Convert temperatures between Fahrenheit and Celsius.
    println!("Converting f {} to c {:.2}", 1.0, convert_f_to_c(1.0));
    println!("Converting f {} to c {:.2}", 100.0, convert_f_to_c(100.0));
    println!("Converting f {} to c {:.2}", -20.0, convert_f_to_c(-20.0));
    // Generate the nth Fibonacci number.
    let x = 32;
    println!("The {}th Fibonacci number is: {}", x, gen_nth_fibonacci(x));
    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
    carol();
}

fn convert_f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn gen_nth_fibonacci(x: u32) -> u32 {
    if x <= 1 {
        return x;
    }
    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=x {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

fn carol() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for (i, _gift) in gifts.iter().enumerate() {
        let day = {
            if i == 0 {
                "first"
            } else {
                gifts[i - 1].rsplit(" ").last().unwrap()
            }
        };
        print!("On the {day} day of Christmas my true love sent to me\n");
        for g in 0..=i {
            println!("{},", gifts[g])
        }
    }
}
