fn main() {
    let mut lines = Vec::new();
    let mut line = String::new();
    while std::io::stdin().read_line(&mut line).is_ok() {
        if line.trim().is_empty() {
            break;
        } else {
            lines.push(line.trim().to_string());
            line = String::new();
        }
    }
    let sum: i32 = lines
        .iter()
        .filter_map(|line| line.split(' ').last()?.replace(',', "").parse::<i32>().ok())
        .sum();
    print!("Total: ");
    let sum = sum.to_string();
    for (i, c) in sum.chars().enumerate() {
        if i > 0 && i % 3 == sum.len() % 3 {
            print!(",");
        }
        print!("{}", c);
    }
    println!();
    println!();
    for line in lines {
        if let Some((line, _)) = line.split_once(" - ") {
            println!("{}", line);
        } else {
            println!("{}", line);
        }
    }
}
