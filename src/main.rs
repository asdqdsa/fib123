fn main() {
    println!("Fib?");
    let mut tempr = String::new();
    std::io::stdin()
        .read_line(&mut tempr)
        .expect("Error, can't read the input");

    let tempr = tempr.trim().parse::<u128>()
        .expect("Error!***#>");
    fib(tempr);
    // println!("{}",fib(tempr));

}

// fn fib(n :u32) -> u32 {
//     // if n == 1 || n == 0 {
//     //     return n;
//     // } else {
//     //     return fib(n - 2) + fib(n - 1)
//     // }

//     match n {
//         0 => n,
//         1 => n,
//         n => fib(n - 2) + fib(n - 1),
//     }
// }

fn fib(mut number: u128) {
    let mut prev_fib :u128 = 0;
    let mut next_fib :u128 = 1;

    // loop fib
    // let mut iter :u128 = 1;
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

    // while fib
    // let mut iter :u128 = 1;
    while number > 0 {
        // (prev_fib, next_fib) = (next_fib , next_fib + prev_fib);
        let mem_num = next_fib;
        next_fib = next_fib + prev_fib;
        prev_fib = mem_num;
        // iter += 1;
        number -= 1;
    }
    println!("{}",next_fib);

        // for fib
    // for _ in 1..number {
    //     (prev_fib, next_fib) = (next_fib , next_fib + prev_fib);
    // }
    // println!("{}",next_fib);
}
