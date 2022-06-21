// Bài tập cho trait
// Đề bài : Implement trait Iterator (của thư viện Rust) cho kiểu dữ liệu Struct sau
// tim day so fibonacci nho hon 400 cach 1
use std::iter::Iterator;
#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}
impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut a = self.a;
        let mut b = self.b;
        self.a = b;
        self.b = a + b;
        Some(a)
    }
}

fn fibonacci_numbers() -> impl Iterator<Item = u32> {
    Fibonacci { a: 0, b: 1 }
}
// tim day so fibonacci nho hon 400 cach 2
fn find_fibonacci(x: u32) -> Vec<i32> {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    let mut d: Vec<i32> = Vec::new();
    for _ in 0..x {
        c = a + b;
        a = b;
        b = c;
        d.push(c);
    }
    d
}


// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime 

use std::fmt;
struct StrDisplayable<'a>(Vec<&'a str>);

impl<'a> fmt::Display for StrDisplayable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;
        }
        Ok(())
    }
}

fn main() {
    println!("*********************Bai tap 1 cach 1*************************\n");
     let mut x = 0;
    for number in fibonacci_numbers() {
        x += 1;
        if x> 2{
            println!("{}", number);
        }
        if x ==  15{
            break;
        }
  
    }
    println!("\n");
    println!("*********************Bai tap 1 cach 2*************************\n");
    let b = find_fibonacci(13);
    println!("{:?}", b);
    println!("\n");
    println!("*************Bai tap 2****************\n");
    let vec: Vec<&str> = vec!["a","bc","def"];
    let vec_Foo = StrDisplayable(vec);
    println!("{}",vec_Foo);

}
