// Primitive Data Type 
// Primitive are Two types


// Scalar
// fn main(){
//     println!("Hello world");
//     // let x :i32 = 2;

//     // integers types (store signed integers)
//     // i8 , 
//     // i16 , 
//     // i32 , 
//     // i64  , 
//     // i128

//     // // for unsigned we use  
//     // u8,

//     // floating point values

//     // f32 , f64,

//     // let floating_point :f32 = 10.9;  // default is f32 in float value


//     // // boole
//     // let true_or_false:bool = true  // we can also say that true is 1 and false is 0


//     // let letter:char = 'v'; //  only contain single and only use single quotes

// }


// Compound
// fn main(){
//     // tuple -> excess element , change element

//     // let tup :(i32 , bool ,char) = (1 , true , 's');
//     // excess element from tuple
//     // println!("{} , {} , {}",tup.0 , tup.1 , tup.2);

//     // let make it mut and let change it
//     // let mut _tup :(i32 , bool ,char) = (1 , true , 's');
//     // // _tup.0 = 10;
//     // _tup = (10 , false , 'k');
//     // println!("{} , {} , {}",_tup.0 , _tup.1 , _tup.2);
// }

use std::io;

fn main(){
    // array
    // let mut arr = [1,2,3,4,5];
    // arr[4] = 10;j
    // println!("{}",arr[4]);


    // // type in array
    // let mut arr2: [i32; 5] = [1,2,3,4,5];

    // println!("{}",arr[4]);


    // sum
    let sum = 5 +10;
    println!("{}",sum);

    let mut num = String::new(); 
    io::stdin()
    .read_line(&mut num)
    .expect("faled to read line");


    let num:u32 = num.trim().parse().expect("Please enter a number");

    // subtraction
    let subtraction :u32 = 10 - num;
    println!("{}",subtraction);
}

