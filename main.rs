fn get_age(age: &str) -> u32 
{
  age.chars().next().unwrap().to_digit(10).expect("Not a NUMBER in 0-9")
}

fn main ()
{
    println!("{:?}", get_age("2 years old"));
}
