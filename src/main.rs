fn digitize(n:u64) -> Vec<u8> {
    n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect::<Vec<u8>>()
}

fn main(){
    let result = digitize(123494835);
    for val in result{
        println!("{val}");
    }
}