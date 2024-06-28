const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 

fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x is now: {x}");
    }

    println!("The value of x is: {x}");
    println!("Constant is: {THREE_HOURS_IN_SECONDS}");
}
