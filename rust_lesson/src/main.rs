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

    say_hello();

    println!("{}", add(1,2));

}

fn say_hello(){
    println!("Hello");
}

fn add(a: i32, b: i32) -> i32{
    return a + b
}
