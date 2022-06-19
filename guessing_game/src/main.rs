use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    rand_guessing();//guessing_game();

    println_xy();
}

#[warn(dead_code)]
fn guessing_game() {
    println!("Guess the number! Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

fn println_xy() {
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);
}

// crate 是一个 Rust 代码包。我们正在构建的项目是一个 二进制 crate，它生成一个可执行文件。
// rand crate 是一个 库 crate，库 crate 可以包含任意能被其他程序使用的代码，但是不能独自执行。
// [dependencies] 表块标题之下添加 rand = "0.8.3"
fn rand_guessing() {
    println!("===V2=== Guess the number! Please input your guess [1,100].");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        // 所以我们必须把从输入中读取到的 String 转换为一个真正的数字类型，才好与秘密数字进行比较。
        // Rust 允许用一个新值来遮蔽 （shadow） guess 之前的值。这允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量，诸如 guess_str 和 guess 之类。
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("secret_number: {}, You guessed: {} ", secret_number, guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}