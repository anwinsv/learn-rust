fn main() {
    println!("Hello, world!");
    another_function();
    print_labeled_measurement(69, 'm');

    //statement vs expression
    let x = 5; //statement
    //let y = (let x = 5); //error: expected expression, found statement
    let z = x + 1; //expression

    let y = {
        let x = 3;
        x + 1 //expression: doesn't have a semicolon at the end
        //if you put a semicolon at the end, it will become a statement
        // and return nothing
    };

    let x = five();
    println!("The value of x is: {x}");

    control_flow();
    loops();
}

//another function
fn another_function() {
    println!("This is another function.");
}

//argument passing
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

//return values
fn five() -> i32 {
    5 //expression: doesn't have a semicolon at the end
    //this is totally valid,
    //but we can also use the return keyword to return
    // a value from a function
    //return 5; //this is also valid
    //5; //this will cause an error because it is a statement
}

//control flow - if statements, loops, and match
fn control_flow() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

//loops - loop, while, for
fn loops() {
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");

    //while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    //liftoff countdown using for loop
    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF!!!");
}