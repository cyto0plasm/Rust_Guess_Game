use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    loop {
        
        let secret = rand::thread_rng().gen_range(1..=100);
        
        println!("Guess a number");
        
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess:u32 = match guess.trim().parse(){
        Ok(num)=> num,
        Err(_) =>{
            println!("this is not a number!");
            continue;
        },
    };
    println!("You Guessed {guess}");
    
    match guess.cmp(&secret) {
        Ordering::Less => println!("\x1b[1;34mToo Small!\x1b[0m"),
        Ordering::Greater => println!("\x1b[1;31mToo Big!\x1b[0m"),
        Ordering::Equal => println!("\x1b[1;32mYou Win\x1b[0m"),
    }
    println!("New Game y/n");
 let mut con = String::new();
 io::stdin().read_line(&mut con).expect("Faild to reed line");
 
 match con.trim().to_lowercase().as_str() {
     "n" => break,
     "y"=>continue,
     _=> println!("Please enter y or n."),
 }  
}

}
