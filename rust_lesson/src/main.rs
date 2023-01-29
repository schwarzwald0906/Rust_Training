fn main() {
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
