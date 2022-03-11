use std::io;

fn main() {
    'outer: loop {
        println!(
            "Type the n in Nth value in the Fibonnacci sequence, and i shall reveal it to you!"
        );

        loop {
            let mut nth = String::new();
            io::stdin().read_line(&mut nth).expect("Value required!");
            let nth: u32 = match nth.trim().parse() {
                Ok(num) => num,
                Err(err) => {
                    println!("{}", err);
                    println!("Invalid value, type again!({})", nth);
                    continue;
                }
            };

            if nth < 1 {
                break 'outer;
            }

            println!(" result: {}", fibo(nth, 1, 0, 1));
            break;
        }
    }
}

fn fibo(pos: u32, current_pos: u32, previous_value: u32, value: u32) -> u32 {
    print!("{} ", previous_value);
    if pos == current_pos {
        return previous_value;
    } else {
        return fibo(pos, current_pos + 1, value + previous_value, previous_value);
    }
}

#[test]
fn test_fibo() {
    assert_eq!(fibo(10, 1, 0, 1), 34);
}
