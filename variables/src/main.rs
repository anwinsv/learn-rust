fn main() {
    let x = 5;
    println!("The value of x (immutable) is: {}", x);
    // x = 6; // This line will cause a compile-time error because `x` is immutable by default
    // println!("The value of x is: {}", x);

    let mut y = 10; // `y` is mutable
    println!("The value of y is: {}", y);
    y = 15; // This is allowed because `y` is mutable
    println!("The value of y after mutation is: {}", y);

    // Shadowing example
    let z = 20;
    println!("The value of z (before shadowing) is: {}", z);
    let z = z + 5; // Shadowing `z` with a new value
    println!("The value of z (after shadowing) is: {}", z);
    {
        let z = z * 2; // Shadowing `z` again in a new scope
        println!("The value of z (inside inner scope) is: {}", z);
    }
    println!("The value of z (after inner scope) is: {}", z);

    //Compound types
    //tupples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {tup:?}");
    let (a, b, c) = tup; //destructuring the tuple into individual variables
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");

    let five = tup.0; // Accessing the first element of the tuple
    println!("The first element of tup is: {five}");
    println!("The second element of tup is: {}", tup.1); // Accessing the second element of the tuple
    println!("The third element of tup is: {}", tup.2); // Accessing the third element of the tuple

    //arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array a = {a:?}");

    let b = [3; 5]; // This creates an array of length 5 where each element is initialized to 3
    println!("Array b = {b:?}");

    let first = a[0]; // Accessing the first element of the array
    let second = a[1]; // Accessing the second element of the array
    println!("The first element of array a is: {first}");
    println!("The second element of array a is: {second}");
}
