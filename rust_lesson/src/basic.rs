
const B: i32 =2 ;
// let c =1;
fn basic() {
    const A: i32 =2 ;
    let a: i32 =1;
    // a=32; 

    let mut b: i32 =2;
}

fn types(){
    //数値型
    let a=1;
    let b=2.0;

    let c=3;

    let d=4.0f32;

    let f = 1 as f64 +2.0;

    //論理型
    // true ,false
    let g=false; 
}
fn array(){
    let t1=(1,true,2.0);
    let t2 =(2.0,1,true);

    println!("{:?}",t1) ;

    let i=t1.0;
    println!("{}",i);

    let(x,y,_)=t2;

    //配列
    let l1=[1,2,3];
    let l2=[0;1000];

    println!("{:?}",l1);

    let i=l1[0];

    // let l3= &l1[0..2];
    let l3= &l1[0..=2];

    println!("{:?}",l3);
}
