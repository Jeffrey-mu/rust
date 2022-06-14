// 使用cargo创建项目 
// cargo new 项目名
// ==========================
// Cargo.toml 配置文件
// edition rust 版本
// dependencies 项目依赖包
// ==========================
// Cargo build --release // flag 优化
// 开发 发布
// Cargo run
// Cargo check

fn main() {
    // let x = 5; // 不可变变量
    // let mut x = 5; // mut 可变变量
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("The value of x is: {}", THREE_HOURS_IN_SECONDS);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    let spaces = "";
    let spaces = spaces.len();
    println!("The spaces len is: {}", spaces);
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);
    let a = [3; 5]; // [3, 3, 3, 3, 3] 数组越界会报错
    println!("{}", a[4]); // 3 
    another_function(1);
}
// 函数命名 小写单词 下划线拼接
// 参数
fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    print_labeled_measurement(5, 'h');
    let y = five() + six();
    println!("The value of y is: {}", y);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}
// 函数返回值 默认返回最后一条表达式 当最后一条为表达式 将会报错
fn five() -> i32 {
    5
}

// 注释三种形态

fn six() -> i32 { // 注释三种形态
    control_flow();
    // 注释三种形态
    6
}

// 流程控制 Control Flow
fn control_flow() {
    // 还值得注意的是，此代码中的条件必须是布尔值。如果条件不是布尔值，我们会得到一个错误。rust 并不会像JavaScript进行隐式转换
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // 您可以通过在 else if 表达式中组合 if 和 else 来使用多个条件。
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // 在 let 语句中使用 if
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {} lin: 95", number);
    repetition(2);
}

// 循环重复  loop, while, and for
fn repetition(x: i32) -> i32 {
    loop {
        println!("again!");
        // break 跳出循环
        // if x == 1 { break }
        // 从循环返回值
        // if x == 2 { break 2}
        if x == 2 { break}

    }
    let mut number = 3;

    while number != 0 {
        // println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
    // rev 来反转范围
    for number in (1..6 ).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    // 变量范围

    { // s 在这里是无效的，它还没有被声明 
        let s = "hello"; // s 从现在开始有效 // 用 s 做事 
        println!("{}!!!", s);
    } // 这个范围现在结束了，s 不再有效
    let mut s = String::from("hello");
    // 这种字符串可以变异：
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
    // 那么，这里有什么不同呢？为什么字符串可以变异而文字不能？不同之处在于这两种类型如何处理内存。
    1
}