const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 

fn main() {
    section_3_2();

    another_function(3);

    section_3_5();

}

fn section_3_5(){
    println!("------------------ Section 3.5 ");

    let number = 7;

    if number < 5 {
        println!("Condition True");
    } else {
        println!("Condition False");
    }

    // if is itself an expression...

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}");

    // expression are really cool...  I re-gain that awesome
    // vibe I loved so much with groovy closures !

    // carefull though, expression must be type consistant

    let condition = true;
    // this line will fail because types are inconsistant 
    // even if the code does not resolve to the wrong type
    //let number = if condition { 5 } else { "six" };

    println!("The value is {number}");

    println!("<< Section 3.5 <<<<<")
}

fn another_function(x: i32){
    println!("The parameter is {x}")
}
fn section_3_2(){

    println!("----------------  Section 3.2 ");
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

    let (_x,y,_z) = tup;

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

    // arrays
    let _a = [1, 2, 3, 4, 5];
    // arrays live on the stack and thus are fixed in size once created
    // array type must include type and lenght when declared
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; 

    // array can be declared as such if they are to contain the same value
    let a = [3; 5]; // creates an array of lenght 5 with all 3

    // array indexing is as one would expect
    let second_element = a[1];
    println!("second element is {second_element}");

    let message = "toto";
    let stuff = [message; 100];
    println!("{}, {}", stuff[0], stuff[1]);

    println!("<< Section 3.2 <<<<<<<<<<<<<<<<<<");
}
