use std::fmt::Debug;
use std::fmt::Display;

// use rand::Rng;
// use test_module::sub_mudule1;
// use rust_lesson::sample_trait::{Shape,Rectangle,Circle, double_area};

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle{
//     fn area(&self)-> u32{
//         self.width*self.height
//     }

//     fn new( width: u32,height: u32) ->Self{
//         Rectangle { width, height }
//     }
// }
fn main() {
    // //Vector
    // let v1=[1,2,3];

    // let mut v3=Vec::new();
    // v3.push(1);
    // v3.push(2);
    // v3.push(3);
    // println!("{:?}",v3);

    // let x =v3.pop();
    // println!("{:?}",x);
    // println!("{:?}",v3);

    // //文字型
    // let c1 ='a';

    // //文字列型
    // let s1="Rust";

    // let mut s2=String::from("Python");
    // s2.push_str(",Rust");

    // say_hello();

    // println!("{}", add(1,2));

    // let x=5;
    // if x > 0{
    //     println!("OK!");
    // }

    // let x =0;
    // match x {
    //     0=> println!("Zero!"),
    //     1=>{
    //         println!("Ome!");
    //         println!("One!");
    //     },
    //     _=>print!("Other!"),
    // };

    // let y = match x {
    //     0=> println!("Zero!"),
    //     1=>{
    //         println!("Ome!");
    //         println!("One!");
    //     },
    //     _=>print!("Other!"),
    // };

    // // loop 
    // let mut cnt=0;
    // loop{
    //     println!("Hello");
    //     if cnt == 10{
    //         break;
    //     }
    //     cnt += 1;
    // }

    // // while 
    // let mut cnt =0;
    // while cnt<=10{
    //     println!("Hello");
    //     cnt += 1;
    // };

    // // for 
    // for i in[1,2,3]{
    //     println!("Hello,{}",i);
    // }
    // let r=1..=10;
    // for x in r{
    //     println!("{}",x * x);
    // }

    // let mut v1 =vec![1,2,3];
    // println!("v1 ptr: {:?}", v1.as_ptr());
    // println!("v1[0] :{:p}", &v1[0]);

    // println!("v1 len: {}", v1.len());
    // println!("v1 capacity: {}", v1.capacity());

    // v1.push(4);
    // println!("v1 ptr: {:?}", v1.as_ptr());
    // println!("v1 len: {}", v1.len());
    // println!("v1 capacity: {}", v1.capacity());

    // println!("v1 ptr: {:?}", v1.as_ptr());
    // let v2=v1.clone();
    // println!("v1 ptr: {:?}", v1.as_ptr());
    // println!("v2 ptr: {:?}", v2.as_ptr());
    
    // let s1=String::from("Hello");
    // let s2=String::from("Rust");
    // let s =concat(&s1, &s2);
    // println!("{}",s1);
    // println!("{}",s2);
    // println!("{}",s);

    // let x=Box::new(1);
    // println!("x: {:p}",x);
    // println!("*x + 2= {}",*x + 2);

    // let a = Rc::new("hello".to_string());
    // println!("count1: {}",Rc::strong_count(&a));
    // {
    //     let b = Rc::clone(&a);
    //     println!("a: {:p}",a);
    //     println!("b: {:p}",b);
    //     println!("count2: {}",Rc::strong_count(&a));
    // }
    // println!("count3: {}",Rc::strong_count(&a));

    // let height =5;
    // let mut rectangle = Rectangle::new(10, 5); 
    // rectangle.height=10;
    // println!("width: {}",rectangle.width);
    // println!("height: {}",rectangle.height);

    // println!("area: {}",rectangle.area());

    // let c=Shape::Circle;
    // let s= Shape::Square(1);
    // let t =Shape::Triangle { base: 10, height: 5 };
    // c.sample_method();
    // s.sample_method();
    // t.sample_method();

    // let v =vec![1,2,3];
    // let val = v.get(0);

    // match val {
    //     // Some(1)=>println!("value is 1"),
    //     // Some(2|3)=>println!("value is 2 or 3"),
    //     Some(x) if *x == 1 =>println!("value is 1"),
    //     Some(x) =>println!("vslue exists: {}", x ),
    //     None => println!("value is None"),
    // }
    // // if let Some(x) =val {
    // //     println!("val={}",x)
    // // }

    // let random_number = rand::thread_rng().gen_range(1..10);
    // print!("{}",random_number);

    // crate::test_module::sub_mudule1::test_fn1();
    // self::test_module::sub_mudule1::test_fn1();

    // let rect = Rectangle{
    //     width: 4.0,
    //     height: 5.0,
    // };
    // let circle =Circle {
    //     radius: 2.0,
    // };

    // println!("Rectangle area is: {}" ,rect.calc_area());
    // println!("Rectangle perimeter is: {}" ,rect.calc_perimeter());
    // Rectangle::do_something();
    // println!("Circle area is: {}" ,circle.calc_area());
    // println!("Circle perimeter is: {}" ,circle.calc_perimeter());
    // Circle::do_something();
    // println!("Rectangle default: {}" ,double_area(&rect));
    // println!("Circle default: {}" ,double_area(&circle));

    let p1 = Point{x:1, y:2};
    // let p2 = Point{x:1.1, y:1.2};
    // let p3 = Point{x:"x", y:"a"};
    // println!("p1.max {:?}",p1.max());
    // println!("p2.max {:?}",p2.max());
    // println!("p3.max {:?}",p3.max());

    p1.print_arg("test");
    p1.print_arg(true);

}

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T: PartialOrd + Debug> Point<T> {
//     fn max(&self) -> &T {
//         if self.x >= self.y {
//             &self.x
//         } else {
//             &self.y
//         }
//     }
//     fn print_arg<U: Display>(&self, val: U){
//         println!("self.x: {:?}" ,self.x);
//         println!("val: {}", val);
//     }
}

// fn max<T> (a:T,b:T) -> T
// where T: PartialOrd + Debug
// {
//     if a >= b {
//         a
//     } else {
//         b
//     }
// }

// mod test_module {
//     pub mod sub_mudule1{
//         pub fn test_fn1(){
//             println!("Hello World1");
//         }
//         fn test_fn2(){
//             println!("Hello Rust1");
//         }
//     }
//     mod sub_mudule2{
//         pub fn test_fn1(){
//             println!("Hello World2");
//         }
//         fn test_fn2(){
//             println!("Hello Rust2");
//         }
//     }
    
// }

// impl Shape { 
//     fn sample_method(&self){
//         println!("call sample_method");
//     }
// }

// enum Shape {
//     Circle,
//     Square(u32),
//     Triangle{base: u32,height: u32},
// }
// fn concat(a: &String, b: &String) -> String {
//     let c =format!{"{}, {}", a, b};
//     c
// }

// fn say_hello(){
//     println!("Hello");
// }

// fn add(a: i32, b: i32) -> i32{
//     return a + b
// }
