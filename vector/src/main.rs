#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}


fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 5;
    }

    for i in v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    for cell in row {
        println!("{:?}", cell);
    }
}
