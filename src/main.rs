use std::fmt::{Display, Formatter};
use std::mem;
fn main() {
    // chapter 2.0 原生类型
    let _logical: bool = true;

    let _a_float: f64 = 1.0; // 常规说明
    let _an_integer = 5i32; // 后缀说明

    let _default_float = 3.0; // 默认说明
    let _default_integer = 7;

    let mut _inferred_type = 12;
    _inferred_type = 4294967296i64; //根据上下文自动推断

    let mut _mutable = 12; //可变变量可以更改值
    _mutable = 21;

    // mutable = true; 不可以更改其类型

    let _mutable = true; //使用遮蔽(shadow)来覆盖前面的变量

    // chapter 2.1 字面量和运算符
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 -2 = {}", 1i32 - 2);

    println!("true AND false = {}", true && false);
    println!("true OR false = {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);

    // chapter 2.2 元组

    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;
        (boolean, integer)
    }

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reverse pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 解构
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?},{:?},{:?},{:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    impl Display for Matrix {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
        }
    }

    println!("{}", matrix);

    fn transpose(tuples: Matrix) -> Matrix {
        Matrix(tuples.0, tuples.2, tuples.1, tuples.3)
    }

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    // chapter 2.3 数组和切片

    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice： {}", slice[0]);
        println!("the slice has {} element", slice.len());
    }

    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

    // 下标从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    // 数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);
}
