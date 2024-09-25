use std::io;

fn main() {
    let mut s=String::new();
     loop {
        println!("Enter the city or ZIP to get Weather details");
        let mut inp=String::new();
        io::stdin().read_line(&mut inp);
        
        //call weather api
     }
}
