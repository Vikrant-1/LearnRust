// Rust has three kinds of loops: [loop, while, for]. 
fn main(){
    // first_loop();
    // first_loop_break();
    // loop_label();
    // loop_while();
    // loop_collection();
    // loop_for_range();

}


//  loop keyword
// fn first_loop(){
//     loop {
//         println!("Again and again");
//     }
// }




// loop key word with break 
// fn first_loop_break(){
//     let mut number  = 5;
//    let res =  loop {
//         number += 1;
//         println!("{number}");
//         if number == 10 {
//             break number *2;
//         };
//     };

//     println!("res is {res}");
// }



// Loop Labels to Disambiguate Between Multiple Loops
// fn loop_label(){
//     let mut count = 0;
//    'counting_up : loop {
//     println!("count = {count}");
//      let mut remaining = 10;
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }

//             remaining -=1;
//        }
//        count +=1
//    }
//     println!("End count = {count}");
// }




// Conditional Loops with while
// fn loop_while(){
//     let mut number:i32 = 10;
//     while number != 0 {
//         println!("number is {number}");
//         number -=1;
//     }
//     println!("LIFTOFF!!!");
// }



// Looping Through a Collection [while , for]

// fn loop_collection(){
//     // const ARR : [i32;5] = [1,2,3,4,5];
//     // let mut index = 0;
//     // while index < 5 {
//     //     println!("Element is {}", ARR[index]);
//     //     index +=1;
//     // }
//     // let a = [10, 20, 30, 40, 50];
//     // for element in a {
//     //     println!("the value is: {element}");
//     // }
// }

//  for loop with range

// fn loop_for_range(){

//     // for num in -10..4 {
//     //     println!("Number is {num}");
//     // }

//     // with reverse method

//     for num in (-10..4).rev() {
//         println!("Number is {num}");
//     }

// }
