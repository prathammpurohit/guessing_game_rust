use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
        let random_number=rand::thread_rng().gen_range(1..=100);
        println!("Random number = {random_number}");

        loop{
                let mut guess=String::new();
                println!("Enter Guess");
                io::stdin()
                .read_line(&mut guess)
                .expect("Falied to read");

                let guess:usize=match guess.trim().parse(){
                        Ok(guess)=>guess,
                        Err(_)=>continue,
                };

                match guess.cmp(&random_number){
                        Ordering::Less=>println!("Small!"),
                        Ordering::Greater=>println!("Big!"),
                        Ordering::Equal=>{println!("Equal! And you WIN!");break;}
                }
        }
}
