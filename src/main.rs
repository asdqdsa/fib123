fn main() {
    println!("Fahrenheit -> Celsius");
    let mut tempr = String::new();
    std::io::stdin()
        .read_line(&mut tempr)
        .expect("Error, can't read the input");

    let tempr = tempr.trim().parse::<u32>()
        .expect("Error!***#>");
    fib(tempr);
}

fn fib(number: u32) {

    // let mut iter :u32 = 1;
    let mut prev_fib = 1;
    let mut next_fib = prev_fib;

    // loop fib
    // loop {
    //     let mem_num = next_fib;
    //     next_fib = next_fib + prev_fib;
    //     prev_fib = mem_num;
    //     iter += 1;
    //     if iter > number {
    //         println!("{}",next_fib);
    //         break
    //     }
    // }

    // for fib
    for _ in 0..number {
        (prev_fib, next_fib) = (next_fib , next_fib + prev_fib);
    }
    println!("{}",next_fib);

    // while fib
    // while iter <= number {
    // (prev_fib, next_fib) = (next_fib , next_fib + prev_fib);
    // iter += 1;
    // }
    // println!("{}",next_fib);
}
