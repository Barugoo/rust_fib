use std::io;

fn get_input_number() -> i32 {
    loop {
        let mut input = String::new();

        io::stdin().
            read_line(&mut input).
            expect("Unable to read stdin");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Incorrect number! Please input the correct one:");
                continue;
            },
        };

        return input;
    }
} 

fn get_n_fib_num(n: i32) -> i32 {
    let mut cur = 1;
    let mut prev = 0;

    for _ in 2..=n {
	let tmp = cur + prev;
        prev = cur;
        cur = tmp;
    }
    cur
}

fn main() {
    println!("Please, input N to get N-th Fib number");

    let input = get_input_number();
    let res = get_n_fib_num(input);
    println!("N-th fib number is: {res}");
}
