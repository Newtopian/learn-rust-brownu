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

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x,y,_z) = tup;

    //println!("the tup is : {tup}");
    // Agove will fail as default formatter does not know how to print tuples
    println!("VAlue of y is {y}");

    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
    println!("Accessing values from tuple {first}, {second}, {third}");

    //but thhis wont work for some reason seems index access to
    //tuple is not always available... interesting
    //println!("Indexed access from string {tup.0}, {tup.1}, {tup.2} -- will fail compilation !")
}
