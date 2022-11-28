fn main() {
    println!("{}", is_pal(1221));
}

fn is_pal(x: i32) -> bool {
    let binding = x.to_string();
    let s: String = binding.chars().rev().collect();
    return x.to_string() == s;
}