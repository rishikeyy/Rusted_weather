use std::{any::{type_name, type_name_of_val}, io, string};

use reqwest::blocking::{Client, Response}; 
use std::collections::HashMap;

fn type_of<T>(_: T) -> &'static str {
   type_name::<T>()
}



 fn api_call(client: &Client,inp: String) -> Result<Response, reqwest::Error>{   

// let url="GET /current.json?q=53.1%2C-0.13 HTTP/1.1
// X-Rapidapi-Key: 1db8c01401msha79269e2ab23f60p1ad899jsndcbac7f7c6f2
// X-Rapidapi-Host: weatherapi-com.p.rapidapi.com
// Host: weatherapi-com.p.rapidapi.com
// ";

let url = "https://weatherapi-com.p.rapidapi.com/current.json";

let mut params=HashMap::new();
params.insert("q",inp);
let res = client
   .post(url)
   .header("X-Rapidapi-Key", "1db8c01401msha79269e2ab23f60p1ad899jsndcbac7f7c6f2")
   .header("X-Rapidapi-Host", "weatherapi-com.p.rapidapi.com")
   .query(&params)
   .send()?;
   //  .text()
   //  .expect("Reason??");
    

   return  Ok((res));
    
}

fn main() {
    let mut s=String::new();
    let  client = reqwest::blocking::Client::new();

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
               //    let resp=api_call(&client,inp);
               //    //let resp_body=resp;

               //    print!("{}",resp);

               // continue;

               match api_call(&client, inp) {
                  Ok(resp) => {
                      // Handle the response body
                      match resp.text() {
                          Ok(resp_body) => {
                              println!("Response Body: {}", resp_body);
                          }
                          Err(e) => {
                              println!("Failed to read response body: {}", e);
                          }
                      }
                  }
                  Err(e) => {
                      println!("API call failed: {}", e);
                  }
              }
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
