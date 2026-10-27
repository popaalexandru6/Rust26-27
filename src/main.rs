// 3
#[derive(Debug)]
enum MathError {
    Overflow,
}

// 4
#[derive(Debug)]
enum CharError {
    Ascii,
    Digit,
    HexDigit,
    Letter,
    Printable,
}

// 1
fn is_prime(n: u16) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n.is_multiple_of(2) { return false; }

    let mut i = 3;
    while i <= n / i {
        if n.is_multiple_of(i) { return false; }
        i += 2;
    }
    true
}

// 1
fn next_prime(x: u16) -> Option<u16> {
    let mut curr = x.checked_add(1)?;
    loop { 
        if is_prime(curr){
            return Some(curr);
        }
        curr = curr.checked_add(1)?;
    }
}

// 2
fn add_panic(a: u32, b: u32) -> u32 {
    a.checked_add(b).expect("Addition overflowed beyond u32")
}

// 2
fn mul_panic(a: u32, b: u32) -> u32 {
    a.checked_mul(b).expect("Multiplication overflowed beyond u32")
}

// 3
fn add_res(a: u32, b: u32) -> Result<u32, MathError> {
    a.checked_add(b).ok_or(MathError::Overflow)
}

// 3
fn mul_res(a: u32, b: u32) -> Result<u32, MathError> {
    a.checked_mul(b).ok_or(MathError::Overflow)
}

// 3
fn do_math_ops(a: u32, b: u32, c: u32) -> Result<u32, MathError> {
    let sum = add_res(a, b)?;
    let result = mul_res(sum, c)?;
    Ok(result)
}

// 4
fn to_uppercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::Letter);
    }
    Ok(c.to_uppercase().next().unwrap_or(c))
}

// 4
fn to_lowercase(c: char) -> Result<char, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::Letter);
    }
    Ok(c.to_lowercase().next().unwrap_or(c))
}

// 4
fn print_char(c: char) -> Result<(), CharError> {
    if c.is_control() {
        return Err(CharError::Printable);
    }
    println!("Char valid: {}", c);
    Ok(())
}

// 4
fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() { return Err(CharError::Ascii); }
    if !c.is_ascii_digit() { return Err(CharError::Digit); }
    Ok(c.to_digit(10).unwrap())
}

// 4
fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() { return Err(CharError::Ascii); }
    if !c.is_ascii_hexdigit() { return Err(CharError::HexDigit); }
    Ok(c.to_digit(16).unwrap())
}

// 4
fn print_error(e: CharError) {
    match e {
        CharError::Ascii => println!("Error."),
        CharError::Digit => println!("Error."),
        CharError::HexDigit => println!("Error."),
        CharError::Letter => println!("Error"),
        CharError::Printable => println!("Error"),
    }
}

// 5
fn parse_and_multiply(digit_str: &str, hex_char: char) -> Result<u32, String> {
    let first_char = digit_str.chars().next().ok_or("String empty")?;
    let a = char_to_number(first_char).map_err(|e| format!("{:?}", e))?;
    let b = char_to_number_hex(hex_char).map_err(|e| format!("{:?}", e))?;
    mul_res(a, b).map_err(|e| format!("{:?}", e))
}

fn main() {
    // 1
    println!("1");
    let mut current_prime = 65500; 
    loop {
        match next_prime(current_prime) {
            Some(p) => {
                println!("Next prime after {} is {}", current_prime, p);
                current_prime = p;
            }
            None => {
                println!("No more primes fit inside a u16. Stopped after {}", current_prime);
                break; 
            }
        }
    }

    // 2
    println!("\n2");
    println!("Success add: 10 + 20 = {}", add_panic(10, 20));
    println!("Success mul: 10 * 20 = {}", mul_panic(10, 20));
    
    // 3
    println!("\n3");
    match do_math_ops(10, 20, 2) { 
        Ok(v) => println!("Success: {}", v),
        Err(e) => println!("Failed: {:?}", e),
    }
    match do_math_ops(u32::MAX, 1, 2) { 
        Ok(v) => println!("Success: {}", v),
        Err(e) => println!("Failed: {:?}", e),
    }

    // 4
    println!("\n4");
    let _ = to_uppercase('a').map(|c| println!("'a' to upper: {}", c));
    let _ = to_lowercase('A').map(|c| println!("'A' to lower: {}", c));
    let _ = char_to_number('5').map(|n| println!("'5' to num: {}", n));
    let _ = char_to_number_hex('f').map(|n| println!("'f' to hex: {}", n));
    
    if let Err(e) = to_uppercase('1') { print_error(e); }
    if let Err(e) = char_to_number('X') { print_error(e); }
    if let Err(e) = print_char('\n') { print_error(e); } 

    // 5
    println!("\n5");
    match parse_and_multiply("4", 'A') { 
        Ok(res) => println!("Succes: 4 * A = {}", res),
        Err(err) => println!(" Error {}", err),
    }
    match parse_and_multiply("4", 'Z') { 
        Ok(res) => println!("Succes: {}", res),
        Err(err) => println!(" Error: {}", err),
    }
}