fn main() {
    println!("Referencing!");

    let s1 = String::from("Hello Referencing");
    
    let len = calculate_lenth(&s1);
println!("The length of '{}' is  {}", s1, len);
}

fn calculate_lenth(s: &String)->usize{
    s.len()
}
