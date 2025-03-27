use std::io;

fn main() {
    println!("Enter the number");
    loop {
        let mut loops = String::new();
        io::stdin()
            .read_line(&mut loops)
            .expect("Failed to read line");
        let loops: u32 = match loops.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let mut fibo: [u64; 3] = [0, 1, 1];
        //println!("{},{},{}",fibo[0],fibo[1],fibo[2]);
        for _ in 0..loops - 2 {
            fibo = [fibo[1], fibo[2], fibo[1] + fibo[2]];
            //println!("{},{},{}",fibo[0],fibo[1],fibo[2]);
        }
        println!("{}", fibo[2]);
        break;
    }
}
