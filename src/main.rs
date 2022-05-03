#![allow(dead_code)]

fn main() {
    // chapter 4.0 变量绑定
    let an_integer = 1u32;
    let a_boolean = true;
    let copied_integer = an_integer;
    let unit = ();
    println!("An integer {:?}", copied_integer);
    println!("A boolean {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;

    // chapter 4.1 可变变量

    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // chapter 4.2 作用域和遮蔽

    let long_lives_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        let long_lives_binding = 5_f32;
        println!("inner long: {}", long_lives_binding);
    }

    println!("outer long: {}", long_lives_binding);

    let long_lives_binding = 'a';
    println!("outer long: {}", long_lives_binding);

    // chapter 4.3 变量先声明
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);
    let another_binding;
    another_binding = 1;
    println!("another binding: {}", another_binding);

    // chapter 4.4 冻结
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        println!("_mutable_integer: {}", _mutable_integer);
    }

    _mutable_integer = 3;
    println!("_mutable_integer: {}", _mutable_integer);
}
