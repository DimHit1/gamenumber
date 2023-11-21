use core::num;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Угодайте число");
    let sec_num = rand::thread_rng().gen_range(1, 101);
    loop {
    println!("Пожалуйста введите свое число");

    let mut number =String::new();
    match io::stdin().read_line(&mut number) {
        Ok(_) => {},
        Err(e) => println!("Ошибка при чтении строки {}", e)
    }
    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    

    println!(" Вы загадали {}", number );
    match  number.cmp(&sec_num) {
        Ordering::Less => println!("Слишком маленькое число!"),
        Ordering::Greater => println!("Слишком большое чисдло!"),
        Ordering::Equal => {println!("Вы выйграли!");
        break;
    }
}
}
}