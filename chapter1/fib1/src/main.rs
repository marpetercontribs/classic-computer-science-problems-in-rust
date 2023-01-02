fn fib1(n: u32) -> u32 {
    fib1(n-1) + fib1(n-2)
  }

fn main() {
    println!("{}",fib1(5));
}


