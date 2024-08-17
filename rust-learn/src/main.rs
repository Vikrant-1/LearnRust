// numbers
// fn main() {
//     let x: i32 = 5;
//     let y: u32 = 1000;
//     let z: f32 = 1000.001;

//     print!("x:{}, y:{}, z:{}", x, y, z);
// }

// boolean
// fn main() {
//     let is_male: bool = false;
//     let is_above_18: bool = true;

//     if is_male {
//         print!("you are a male");
//     } else {
//         print!("You are not a male");
//     }

//     if is_male && is_above_18 {
//         print!("You are a legal male");
//     }
// }

// fn main() {
//     let is_male: bool = false;
//     let is_above_18: bool = true;

//     if is_male {
//         print!("you are a male");
//     } else {
//         print!("You are not a male");
//     }

//     if is_male && is_above_18 {
//         print!("You are a legal male");
//     }
// }

// string
fn main() {
    let x = String::from("my name is vikrant"); 
    let char1 = x.chars().nth(10);
    print!("{}",char1.unwrap());
}
