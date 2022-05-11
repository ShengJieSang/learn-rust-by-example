#![allow(dead_code)]

fn create_box() {
    let _box1 = Box::new(3i32);
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy)]
struct Points {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'q';

    let ref ref_c = c;

    let ref_c1 = &c;

    println!("c is equal c1 ? {}", *ref_c == *ref_c1);

    let point = Points { x: 0, y: 0 };
    // 在解构一个结构体时 `ref` 同样有效。
    let _copy_of_x = {
        // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
        let Points {
            x: ref ref_to_x,
            y: _,
        } = point;

        // 返回一个 `point` 的 `x` 字段的拷贝。
        *ref_to_x
    };

    // `point` 的可变拷贝
    let mut mutable_point = point;

    {
        // `ref` 可以与 `mut` 结合以创建可变引用。
        let Points {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;

        // 通过可变引用来改变 `mutable_point` 的字段 `y`。
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    let mutable_borrow = &mut point;

    println!(
        "{} {} {}",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    mutable_borrow.x = 5;
    println!("{}", point.x);

    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    let mut mutabook = immutabook;

    borrow_book(&immutabook);

    borrow_book(&mutabook);

    new_edition(&mut mutabook);

    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);
    }

    for _ in 0u32..1_000 {
        create_box()
    }

    let x = 5u32;
    let y = x;

    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);

    println!("a contains {}", a);

    let b = a;
    // println!("a contains {}", a);

    destroy(b);

    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);

    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 4;
    println!("mutable_box contains {}", mutable_box);

    let boxed_i32 = Box::new(5i32);

    let stacked_i32 = 6i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        eat_box_i32(boxed_i32);
    }
}

fn borrow_i32(borrow_i32: &i32) {
    println!("This int is: {}", borrow_i32);
}
fn destroy(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}
