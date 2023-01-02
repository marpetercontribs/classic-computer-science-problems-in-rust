fn fib2(n: u32) -> u32 {
    if n<2 {
        // println!("fib({}) -> {}",n,n);
        n
    } else {
        // println!("fib({}) -> fib({}), fib({})",n,n-1,n-2);
        fib2(n-1) + fib2(n-2)        
    }
  }

fn main() {
    println!("{}",fib2(5));
    println!("{}",fib2(10));
}
