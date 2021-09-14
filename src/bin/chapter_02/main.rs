use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o numero!");

    let secret_value = rand::thread_rng().gen_range(1..101);

    println!("O numero secreto e: {}", secret_value);

    loop {
        println!("Por favor digite um numero.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Erro ao ler o input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite: {}", guess);

        match guess.cmp(&secret_value) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Voce ganhou!");
                break;
            }
        }
    }
}
