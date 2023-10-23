use std::io;
// struct Operation<T, U> {
//     input1: T,
//     input2: U,
// }

// impl<T,U> Operation<T,U> {

//     fn add(&self){
//         self.input1 + self.input2;
//     }
    
// }

fn main() {

    println!("Enter number 1");
    let num1 = read_int();
    println!();
    println!("Enter number 2");
    let num2 = read_int();
    println!();
    println!("What would you like to do?");
    println!("1.Add");
    println!("2.Subtract");
    println!("3.Divide");
    println!("4.Multiply");
    let choice = read_int();
    println!();

    if choice == 1 {
        println!("The answer for {} + {} is {} ",num1,num2,num1 + num2);
    }
    else if choice == 2{
        println!("The answer for {} - {} is {} ",num1,num2,num1 - num2);
    }
    else if choice == 3{
        println!("The answer for {} / {} is {} ",num1,num2,num1 / num2);
    }
    else if choice == 4{
        println!("The answer for {} * {} is {} ",num1,num2,num1 * num2);
    }
    else{
        println!("Cant do anything");
    }

}

fn read_int() -> u32{
    let mut input  = String::new();
    io::stdin()
    .read_line(&mut input).expect("");
let number = input.trim().parse().expect("number");
number
}
