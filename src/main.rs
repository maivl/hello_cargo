mod generator;
  
fn main() {  
    let n = generator::gen_ran();
    println!("Random u8: {}", n)
}