fn main() {
    let x = five();
    let y = plus_one(10);
    println!("value is x : {x} , y :{y}");
    

}

// Parameters
fn another_function(x: i32, unit_label: char) {
    println!("Another function parameter {x},{unit_label}");
}

// expression and statements

fn sub_main() {
    let y:i32 = {
        let x:i32 = 6;
        x + 1
    };
    println!("The value of y is:{y}");
}

// Functions with Return Values
fn five() -> i32 {
    5
}

fn plus_one(x:i32)->i32{
    x+1
}
