fn main() {
    // chapter 2.0
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
}
