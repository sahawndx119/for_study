//in the name of God

fn main () {
    let mut imput = String::new();
    std::io::stdin().read_line(&mut imput).unwrap();

    println!("hello dear {imput}");
}