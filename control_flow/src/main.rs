fn main() {
    let mut number = 4;
    if number!=0
    {
        println!("the value of number is {}",number);
    }
    let condition = true;
    number = if condition {10} else {12};
    if number % 4 ==0{
        println!("number is divisible by 4");
    }
    else if number % 3 == 0{
        println!("number is divisible by 3");
    }
    else if number % 2 == 0{
        println!("number is divisible by 2");
    }
    else
    {
        println!("number is not divisible by 4,3, or 2");
    }
}
