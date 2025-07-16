fn main() {
    let mut counter = 0;
    let _result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {_result}");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count ={count}");
    let a = [10,20,30,40,50,60,70,80];
    let mut index = 0 ;
    while index<8
    {
        println!("the value is : {}",a[index]);
        index=index+1;
    }
    for element in a{
        println!("the value is :{element}");
    }
    for num in (1..4).rev(){
        println!("{num}!");
    }
}
