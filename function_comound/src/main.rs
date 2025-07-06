fn main() {
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup1;
    println!("value of a,b,c is {},{},{}", a, b, c);
    {
        let a: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = a.0;
        println!("five_hundred is {}(索引访问)or{}(解构)", five_hundred, a.0);
    }
    println!("变量遮蔽！{a}");
    let arr = [1, 2, 3, 4];
    print_array_for_loop(&arr);
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    print_array_for_loop(&arr1);
    let arr2 = [0; 5];
    print_array_for_loop(&arr2);
    let h = 300;
    a_simple_funtion(h);
    let t = five();
    a_simple_funtion(t);
}
fn five() -> i32 {
    5
}
fn a_simple_funtion(x: i32) {
    println!("The key is {x}");
}
fn print_array_for_loop(arr: &[i32]) {
    for element in arr {
        println!("元素: {}", element);
    }
}
