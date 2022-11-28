fn main() {
    println!("{}", ri("MCMXCIV".to_string()));
}

fn ri(s: String) -> i32 {
    let mut x = 0;
    let mut prev = 0;
    let s: Vec<i32> = s.chars().map(|l| {
        if l == 'I' { return 1; }
        else if l == 'V' { return 5; }
        else if l == 'X' { return 10; }
        else if l == 'L' { return 50; }
        else if l == 'C' { return 100; }
        else if l == 'D' { return 500; }
        else if l == 'M' { return 1000; }
        0
        
        // if else if time efficient
        // match is space efficient
        // atleast in leetcode

        // match l {
        //     'I' => 1,
        //     'V' => 5,
        //     'X' => 10,
        //     'L' => 50,
        //     'C' => 100,
        //     'D' => 500,
        //     'M' => 1000,
        //     _ => 0,
        // }
    }).collect();

    for i in s {
        if prev < i { x += i - 2 * prev; }
        else { x += i; } 
        prev = i
    }

    return x;
}