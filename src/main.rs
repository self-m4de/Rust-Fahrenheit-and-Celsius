use std::io;

fn main() {
    println!("Type 'cel' for celsius or 'far' for fahrenheit");
    let mut unit = String::new();
    io::stdin().read_line(&mut unit)
        .expect("Failed to read line");
    let unit: String = match unit.trim().parse() {
        Ok(unit) => unit,
        Err(_) => unit,
    };

    println!("Enter the temperature");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp)
        .expect("Failed to read line");
    let temp: u32 = match temp.trim().parse() {
        Ok(temp) => temp,
        Err(_) => 1,
    };

    let temp = temp as f64;
    convert(unit, temp);

}

// Formulas
// C = (F - 32) * 5/9
// F = 32 + (9/5)C

fn convert(unit: String, temp: f64) {
    if unit == "cel" {
        let fahrenheit: f64 = 32.0 + (9.0 * temp / 5.0);
        println!("{}°F", fahrenheit);
        } 
    else if unit == "far" {
        let celsius: f64 = (temp - 32.0) * 5.0/9.0;
        println!("{}°C", celsius);
        }
    else {
        println!("Please enter 'cel' for celsius or 'far' for fahrenheit");
    }

}

