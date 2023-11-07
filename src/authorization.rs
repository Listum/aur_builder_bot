use std::fs::File;
use std::io::{Read, Write};

fn write(data: Vec<i64>) -> Result<(), Box<dyn std::error::Error>>{
    match File::create("auth") {
        Ok(mut file) => {
            for num in data {
            file.write_all(format!("{}\n", num).as_bytes())?; }
        }
        Err(_) => {}
    }
    Ok(())
}

fn read() -> Result<Vec<i64>, Box<dyn std::error::Error>>{
    let mut file = File::open("auth")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let ids: Vec<i64> = content
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    Ok(ids)
}

pub fn add(id: i64) -> Result<(), Box<dyn std::error::Error>>{
    if check(id) {} else {
    match read() {
        Ok(ids) => {
            let mut array: Vec<i64> = ids;
            array.push(id);
            write(array)?;
        }
        Err(e) => {
            println!("Error reading from file: {}, recreating auth", e);
            init(id)?;
        }
    }}
    Ok(())
}

pub fn check(id: i64) -> bool {
    let mut valid: Option<bool> = None;
    match read() {
        Ok(ids) => {
            let array: &[i64] = ids.as_slice();
            if array.contains(&id) {
                valid = Some(true);
            } else {
                valid = Some(false);
            }
        }
        Err(e) => {
            println!("Error reading from file: {}, recreating auth", e);
            init(id).expect("init");
        }
    }
    return valid.unwrap();
}


fn init(id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("auth")?;
    let vec: Option<Vec<i64>> = Some(Vec::from([id]));
    for num in vec.unwrap() {
        file.write_all(format!("{}\n", num).as_bytes())?;
    }
    Ok(())
}