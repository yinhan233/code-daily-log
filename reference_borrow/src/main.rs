fn main() {
    let s1 = String::from("wooo");
    let (s2,len)=calculate_length(s1);
    println!("the s2 is {s2},the length is {len}");
    //s1 is droped,the reference is below
    let s1 = String::from("woooo");
    let len2 = calculate_length2(&s1);
    println!("here is len2:{len2},and s1 is not drop:{s1}");
    //mut reference can change the value
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");
}
fn calculate_length(astring:String)->(String,usize)
{
    let length=astring.len();
    (astring,length)
}
fn calculate_length2(s:&String)->usize
{
    s.len()
}
fn change(astring:&mut String)
{
    astring.push_str(",world");
}