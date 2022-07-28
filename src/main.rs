



fn main() {
    println!("the sum is {}", 80 + 30);
    println!("{}",(23 - 6) % 5 + 20 * 30 / (3 +4));
    println!("the sum is {}", 80.3 + 34.9);
    print!("{} + ", 80);
    print!("{} =", 34);
    print!(" {}", 80 + 34);
    println!(" \
    {}",2.7 + 1.);
    println!("{}","this \
    is \
    just one line");
    let mut number = 12;
    println!("{}", number);
    number=53;
    println!("{}",number);
    let truth=true;
    let falsity=false;
    print!("{truth} and {falsity}");
    let truth= 5 > 2;
    let falsity= -12.3 >= 10.;
    println!("{} {} {}", truth, falsity, -2 > 0);
    
}
