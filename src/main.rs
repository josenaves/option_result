
fn main() {

    // enum Option<T> {
    //     Some(T),
    //     None
    // }

    let number = -4.0;
    let square_root = find_square_root(number);

    match square_root {
        Some(value) => println!("The square root of {} is {}", number, value),
        None => println!("The square root of {} is not a real number!", number),
    }

    let another_number = 64.0;
    match find_square_root(another_number) {
        Some(value) => println!("The square root of {} is {}", another_number, value),
        None => println!("The square root of {} is not a real number!", another_number), 
    }

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    match divide(4.0,0.0) {
        Ok(value) => println!("The result of {} divided by {} is {}", 4, 0, value),
        Err(value) => println!("Error: {}", value)
    }

    match divide(1024.0,128.0) {
        Ok(value) => println!("The result of {} divided by {} is {}", 1024.0, 128.0, value),
        Err(value) => println!("Error: {}", value)
    }

    // -----------------------

    let base = get_from_database("base");
    let height = get_from_database("height");
    let area_result = calculate_triangle_area(base, height);
    
    match area_result {
        Ok(area) => println!("The area of triangle is {} square units", area),
        Err(error_message) => println!("Error {}", error_message),
    }

    match calculate_triangle_area(get_from_database("base"), None) {
        Ok(area) => println!("The area of triangle is {} square units", area),
        Err(error_message) => println!("Error {}", error_message),
    }

    match calculate_triangle_area(None, None) {
        Ok(area) => println!("The area of triangle is {} square units", area),
        Err(error_message) => println!("Error {}", error_message),
    }
}

fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

fn divide(divisor: f64, dividend:f64) -> Result<f64, String> {
    if dividend == 0.0 {
        Err("Divison by zero is not allowed".to_string())
    } else {
        Ok(divisor/dividend)
    }
}

fn get_from_database(key: &str) -> Option<f64> {
    let database: [(&str, Option<f64>); 2] = [("base", Some(4.0)), ("height", Some(6.0))];

    for (k, v) in database {
        if k == key  {
            return v;
        }
    }
    None
}

fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
    match (base, height) {
        (Some(b), Some(h)) => {
            if b <= 0.0 || h <= 0.0 {
                Err("Both base and height must be positive numbers".to_string())
            } else {
                Ok(0.5 * b * h)
            }
        }
        (None, _) => Err("The base is missing".to_string()),
        (_, None) => Err("The height is missing".to_string())
    }
}