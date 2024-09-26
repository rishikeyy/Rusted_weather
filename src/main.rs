use std::{any::{type_name, type_name_of_val}, io};

fn type_of<T>(_: T) -> &'static str {
   type_name::<T>()
}
fn main() {
    let mut s=String::new();
     loop {
        println!("Enter the city or ZIP to get Weather details");
        let mut inp=String::new();
        io::stdin().read_line(&mut inp);

          //input is zipcode
         match inp.trim().parse::<i64>() {   //type_of(&inp.trim().parse::<i64>())
             Ok(_) => {
               if  inp.trim().chars().count()==5 {
               println!("zip option\n")
                  //call weather api
               continue;
            }

             }
             Err(_) => {
               print!("error parsing\n")
             }
        }

        //input is city,town etc.
        {
         
        }

        
        
        //call weather api
     }
}
