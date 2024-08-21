fn main(){
    first_program();
    check_bool();
    multiple_condition(7);
    handle_if_in_let();
    handle_if_in_let_mismatch_type();
}


// if else condition

fn first_program(){
    let number = 7;
    
    if number < 5 {
        println!("Yes condition is true");

    }else{
        println!("Yes condition is false");
    }
}

// condition must be a bool value
fn check_bool(){
    let number :i32 = 10;
    if number == 2 {
        println!("In a if condition");
    }
}

//Handling Multiple Conditions with else if

fn multiple_condition(number:i32){
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    }else if number % 3 == 0 {
        println!("Number is divisible by 3");
    }else if number % 2 == 0 {
        println!("Number is divisible by 2");
    }else{
        println!("Number is not divisible by 2 , 3 ,4");

    }

}

// Using if in a let Statement

fn handle_if_in_let(){
    let a:i32 = 5;
    let number :i32 =  if a < 4 {10} else {a};
    println!("You number is {number}");
}

// Using if in a let Statement 
// types are mismatched
fn handle_if_in_let_mismatch_type(){
    let a:i32 = 5;
    let result =  if a < 4 {"Yes"} else {1};
    //  get error -> expected `&str`, found integer

    println!("You result is {result}");
}