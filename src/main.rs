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

    println!("Enter  Numbers");
    read
    println!("What would you like to do?");
    println!("1.Add");
    println!("2.Subtract");
    println!("3.Divide");
    println!("4.Multiply");

}

fn read_int(){
    let mut input  = String::new();
    io::stdin()
    .read_line(&mut input).expect("");
input.trim();
}
