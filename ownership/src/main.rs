fn main() {
    //scope
    // s is not valid here, it’s not yet declared
    // let s = "hello"; // s is valid from this point forward

    //string type
    // let s = String::from("hello");
    let mut s = String::from("hello"); //can be mutable
    s.push_str(", world!"); // push_str() appends a literal to a String
    {
        //memory is returned once the variable is out of scope
        s = String::from("hello from another scope");
        println!("{s}");
    } //s scope ends here

    s = String::from("hello from the main scope");
    println!("{s}");

    interaction_with_move();

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.
} // s is valid until the end of this scope
// Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.

//variables and data interaction with move
fn interaction_with_move() {
    // let x = 5; //x binds to 5 and y binds to the same value
    // let y = x; //copy occurs here, both x and y are valid

    let s1 = String::from("hello"); //s1 contains ptr, len, capacity
    let s2 = s1; //s2 doesn't copy the data, just the ptr, len, capacity
    //println!("{s1}, world"); //this can cause error as s1 is no longer the owner
    //ownership is moved to s2, s1 is no longer valid
    println!("{s2}, world"); //this is valid as s2 is the owner of the data
    //this removes the possibility of double free error

    let mut s = String::from("hello");
    println!("{s}, world");
    s = String::from("ahoy"); //the first string is dropped and allocated for second string
    println!("{s}, world");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.