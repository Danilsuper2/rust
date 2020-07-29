use std::io;

fn main() {
    println!("Калькулятор от Дани!");
    let num_1: i32;
    let num_2: i32;

    loop {
        println!("Первое число:");

        let mut str = String::new();
        io::stdin().read_line(&mut str)
            .expect("Ошибка к чтение полоска");

        num_1 = match str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        break
    };

    loop {
        println!("Второе число:");

        let mut str = String::new();
        io::stdin().read_line(&mut str)
            .expect("Ошибка к чтение полоска");

        num_2 = match str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        break
    };

    println!("Сумма: {}", num_1 + num_2);

    test()
}

fn test() {
    println!("test")
}