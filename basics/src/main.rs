use std::io;

//Current Plan: Move everything from the old project into this,
////And then figure out how to link them into this.
fn main() {

    println!("Hello!\n");
    println!("the key word 'let' is used to declare variables.");
    println!("they key word 'mut' makes variables mutable.\n");

    println!("Mutable vs Immutable: \n");
    println!("For Immutables like 'let x = 5' cannot be assigned twice.");
    println!("For Mutables like 'let mut x = 5' can be assigned twice or more.\n");

     let mut mute = 5; //Let's say this is x
    println!("The value of x is: {mute}");
    mute = 6;
    println!("The value of x is: {mute}\n");

    println!("Shadowing: \n");
    println!("It works by overshadowing the first variable with 'let'.\n");

        let shadow = 5;//Let's say this is x

    let shadow = shadow + 1;

    {
        let shadow = shadow * 2;
        println!("The value of x in the inner scope is: {shadow}");
    }

    println!("The value of x is: {shadow}\n");
}
