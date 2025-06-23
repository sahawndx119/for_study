//in the name of God

fn main () {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!("hello dear {}" , input.trim());


    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).unwrap();

    println!("Got another information from you! it is >> {}" , input2.trim())
}