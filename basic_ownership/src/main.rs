fn main() {
    let mut s1 =String::from("hello");
    s1.push_str(",world!");
    println!("{s1}");
    let mut s2 = s1;
    s2.push_str("yoyoyo~");
    println!("{s2}");
    let mut s3=s2.clone();
    s3.push_str("nihao");
    println!("{s2},{s3}");
    let s = String::from("Test");
    take_ownership(s);
    let a:i32 = 113;
    makes_copy(a);
    println!("I am {a}");
    let s4 = gives_ownership();
    let s5 = String::from("ADDBDG"); //s5 is dropped or moved?
    let s6 = takes_and_gives_back(s5);
    println!("s4 is here :{s4},s6 is here :{s6}");
}
fn take_ownership(astring: String)
{
    println!("{astring},which is droped");
}
fn makes_copy(ainteger: i32)
{
    println!("{ainteger},which is copyed");
}
fn gives_ownership() -> String
{
let some_string = String::from("wooooo~");
some_string
}
fn takes_and_gives_back(a_string:String)->String
{
    a_string
}