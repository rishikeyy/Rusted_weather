use std::{any::{type_name, type_name_of_val}, io, string};
use reqwest::Client;
use std::collections::HashMap;

fn type_of<T>(_: T) -> &'static str {
   type_name::<T>()
}



 fn api_call(client: &Client,inp: String){

let url="GET /current.json?q=53.1%2C-0.13 HTTP/1.1
X-Rapidapi-Key: 1db8c01401msha79269e2ab23f60p1ad899jsndcbac7f7c6f2
X-Rapidapi-Host: weatherapi-com.p.rapidapi.com
Host: weatherapi-com.p.rapidapi.com
";

let mut params=HashMap::new();
params.insert("q",inp);
let res = client.post(url)
    .query(&params)
    .send()
    

   return res;
    
}

fn main() {
    let mut s=String::new();
    let  client = reqwest::Client::new();

     loop {
        println!("Enter the city or ZIP to get Weather details");
        let mut inp=String::new();
        io::stdin().read_line(&mut inp);

          //input is zipcode
         match inp.trim().parse::<i64>() {   //type_of(&inp.trim().parse::<i64>())
             Ok(_) => {
               if  inp.trim().chars().count()==5 {
               println!("zip option\n");

                  //call weather api
                  let resp=api_call(&client,inp);
                  print!("{resp}");

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

     }
}
