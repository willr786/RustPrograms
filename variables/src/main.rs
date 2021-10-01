fn main() {

    //Mutability
    //let x = 5 This will cause an error;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //Shadowing
    let x = 5;
    
    let x = x * 1;

    let x = x * 2;

    println!("The value of x i: {}", x);

    //Tuples
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
