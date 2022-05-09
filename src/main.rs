#![allow(dead_code)]

use std::fmt::Debug;

struct A;
struct Single(A);
struct S(A);
struct SGen<T>(T);
struct SingleGen<T>(T);
fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

struct Empty;
struct Null;
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

trait HasArea {
    fn area(&self) -> f64;
}
#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.length
    }
}

struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

struct Container(i32, i32);

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn main() {
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');

    generic::<char>(SGen('a'));
    generic(SGen('b'));

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 'a' };

    println!("{}, {}", x.value(), y.value());

    let empty = Empty;
    let null = Null;
    empty.double_drop(null);

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    println!("Area: {}", &rectangle.area());

    let age = Years(19);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));

    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!(
        "{}{}{}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("{},{}", container.first(), container.last());
    println!("{}", difference(&container));
}
