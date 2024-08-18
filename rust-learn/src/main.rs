// variables , constants , shadowing , constant shadowing ,shadowing in scope




// variables
// fn main (){
//     let mut x:i8 = 5;
//     print!("The Value of x is :{}" , x);

//     x = 6;
//     print!("The Value of x is :{}" , x);
// }

// shadowing

// within same scope variables shadowing
fn main() {
    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    // //  shadowing allow us change type
    // let spaces = "    "; // it is string type and spaces is there
    // let spaces = spaces.len(); // it is number type and lenght of spaces is there.
    // print!("spaces is :{}",spaces);

    // // we can not change type of mutable variable
    // let mut spaces = "  "; // it is type string and mutable var
    // spaces = spaces.len(); // now it become int type
    // // error -> expected `&str`, found `usize`;

    // Constants -> we get warning when we not use uppercase BUT code will exicute
    // to check this change to snakecase
    // we can not mut const
    // the type of the value must be annotated
    // can not use shadowing in same scope
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // print!("three_hours_in_secounds is {}", THREE_HOURS_IN_SECONDS);
    // {
    //     // this code will work
    //     const THREE_HOURS_IN_SECONDS: u32 = 5;
    //     print!("three_hours_in_secounds is {}", THREE_HOURS_IN_SECONDS);
    // }
    //  throw error
    // const THREE_HOURS_IN_SECONDS: u32 = 5;
    // print!("three_hours_in_secounds is {}", THREE_HOURS_IN_SECONDS);

}
