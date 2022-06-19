use std::io;

fn main() {

    //变量和可变性 常量 遮蔽
    mutable_mut_constant();

    //标量类型 数字运算 复合类型 元组 数组
    data_type();

    //函数
    another_function();

    // 控制流 if loop while for
    condition();
}

fn condition() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = if number == 3 { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    //从循环返回
    // loop 的一个用例是重试可能会失败的操作，比如检查线程是否完成了任务。然而你可能会需要将操作的结果从循环中传递给其它的代码。
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // while 条件循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 使用 for 遍历集合
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // for 循环来倒计时的例子
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function() {
    // 在函数签名中，必须声明每个参数的类型。
    print_labeled_measurement(5, 'h');

    println!("The value of x is: {}", plus_one(5));
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn data_type() {
    //Rust 的每个值都有确切的数据类型（data type），该类型告诉 Rust 数据是被指定成哪类数据，从而让 Rust 知道如何使用该数据。
    // 静态类型
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    // 标量类型
    // 标量（scalar）类型表示单个值。Rust 有 4 个基本的标量类型：整型、浮点型、布尔型和字符。

    //每个有符号类型规定的数字范围是 -(2n - 1) ~ 2n - 1 - 1，其中 n 是该定义形式的位长度。
    let x = 32;// i32
    let x: u8 = 254;
    // let x: u8 = x+2; //  attempt to compute `254_u8 + 2_u8`, which would overflow
    //整型溢出:
    //1、当在调试（debug）模式编译时，Rust 会检查整型溢出，若存在这些问题则使程序在编译时 panic。Rust 使用 panic 这个术语来表明程序因错误而退出。第 9 章 “panic! 与不可恢复的错误”会详细介绍 panic。
    //2、在当使用 --release 参数进行发布（release）模式构建时，Rust 不检测会导致 panic 的整型溢出。相反当检测到整型溢出时，Rust 会进行一种被称为二进制补码包裹（two’s complement wrapping）的操作。简而言之，大于该类型最大值的数值会被“包裹”成该类型能够支持的对应数字的最小值。比如在 u8 的情况下，256 变成 0，257 变成 1，依此类推。
    println!("x is: {}", x);


    //浮点数（floating-point number）是带有小数点的数字，在 Rust 中浮点类型（简称浮点型）数字也有两种基本类型。
    //Rust 的浮点型是 f32 和 f64，它们的大小分别为 32 位和 64 位。默认浮点类型是 f64，因为在现代的 CPU 中它的速度与 f32 的几乎相同，但精度更高。
    //所有浮点型都是有符号的。
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Rust 的所有数字类型都支持基本数学运算：加法、减法、乘法、除法和取模运算。整数除法会向下取整。
    // addition
    let _sum = 5 + 10;
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0
    // remainder
    let _remainder = 43 % 5;

    let _t = true;
    let _f: bool = false; // with explicit type annotation
    // if this is intentional, prefix it with an underscore: `_f`


    // Rust 的 char（字符）类型是该语言最基本的字母类型;
    // 我们声明的 char 字面量采用单引号括起来，这与字符串字面量不同，字符串字面量是用双引号括起来。Rust 的字符类型大小为 4 个字节，表示的是一个 Unicode 标量值，
    // 这意味着它可以表示的远远不止是 ASCII。标音字母，中文/日文/韩文的文字，emoji，还有零宽空格(zero width space)在 Rust 中都是合法的字符类型。
    // Unicode 值的范围为 U+0000 ~ U+D7FF 和 U+E000~U+10FFFF。
    // 不过“字符”并不是 Unicode 中的一个概念，所以人在直觉上对“字符”的理解和 Rust 的字符概念并不一致。
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // 复合类型（compound type）可以将多个值组合成一个类型。Rust 有两种基本的复合类型：元组（tuple）和数组（array）。
    // 元组类型 元组是将多种类型的多个值组合到一个复合类型中的一种基本方式。元组的长度是固定的：声明后，它们就无法增长或缩小。
    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {:#?}", x);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    //没有任何值的元组 () 是一种特殊的类型，只有一个值，也写成 ()。该类型被称为单元类型（unit type），该值被称为单元值（unit value）。如果表达式不返回任何其他值，就隐式地返回单元值。

    // 与元组不同，数组的每个元素必须具有相同的类型。与某些其他语言中的数组不同，Rust 中的数组具有固定长度。
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 这里，i32 是每个元素的类型。分号之后，数字 5 表明该数组包含 5 个元素。
    let a = [3; 5]; // 变量名为 a 的数组将包含 5 个元素，这些元素的值初始化为 3。这种写法与 let a = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => 2,
    };

    let element = a[index]; // index out of bounds: the len is 5 but the index is 7

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn mutable_mut_constant() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // cannot assign twice to immutable variable

    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    //常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变。常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须注明。
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    //可以声明和前面变量具有相同名称的新变量。Rustacean 说这个是第一个变量被第二个变量遮蔽（shadow），
    //这意味着当我们使用变量时我们看到的会是第二个变量的值。
    let x = 5;
    let x = x + 1; // x=6
    {
        let x = x * 2; // x=12
        println!("The value of x in the inner scope is: {}", x);
    }// x=6
    println!("The value of x is: {}", x);
    //遮蔽和将变量标记为 mut 的方式不同，因为除非我们再次使用 let 关键字，否则若是我们不小心尝试重新赋值给这个变量，我们将得到一个编译错误。
    //mut 和遮蔽之间的另一个区别是，因为我们在再次使用 let 关键字时有效地创建了一个新的变量，所以我们可以改变值的类型，但重复使用相同的名称。
}
