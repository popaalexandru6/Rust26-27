use std::fs;
use std::io;

// P1 
fn p1(filepath: &str) -> io::Result<()> {
    let content = fs::read_to_string(filepath)?;
    
    let mut max_bytes_line = "";
    let mut max_chars_line = "";
    let mut max_bytes = 0;
    let mut max_chars = 0;

    for line in content.lines() {
        let bytes = line.len();
        let chars = line.chars().count();

        if bytes > max_bytes {
            max_bytes = bytes;
            max_bytes_line = line;
        }
        if chars > max_chars {
            max_chars = chars;
            max_chars_line = line;
        }
    }

    println!("most BYTES ({}): {}", max_bytes, max_bytes_line);
    println!("most characters ({}): {}", max_chars, max_chars_line);
    
    Ok(())
}

// P2
fn rot13(c: char) -> char {
    if c.is_ascii_lowercase() {
        (((c as u8 - b'a' + 13) % 26) + b'a') as char
    } else if c.is_ascii_uppercase() {
        (((c as u8 - b'A' + 13) % 26) + b'A') as char
    } else {
        c
    }
}

fn p2(input: &str, output: &str) -> Result<(), String> {
    let content = fs::read_to_string(input).map_err(|e| e.to_string())?;
    let mut result = String::new();

    for c in content.chars() {
        if !c.is_ascii() {
            return Err(format!("Eroare: Caracter non-ASCII detectat '{}'. Ne oprim.", c));
        }
        result.push(rot13(c));
    }

    fs::write(output, &result).map_err(|e| e.to_string())?;
    Ok(())
}

// P3 
fn p3(filepath: &str) -> io::Result<()> {
    let content = fs::read_to_string(filepath)?;
    
    let abbreviations = [
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna"),
    ];

    let mut words: Vec<&str> = Vec::new();
    
    for word in content.split(' ') {
        let mut replaced = word;
        for &(abbr, full_word) in &abbreviations {
            if word == abbr {
                replaced = full_word;
                break;
            }
        }
        words.push(replaced);
    }

    let result = words.join(" ");
    println!("{}", result);
    
    Ok(())
}


// P4 
fn p4() -> io::Result<()> {
    // Citim fisierul local de test in loc de cel din Windows
    let path = "hosts.txt"; 

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            println!("Nu am gasit hosts.txt: {}", e);
            return Ok(());
        }
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 {
            let ip = parts[0];
            let host = parts[1];
            println!("{} => {}", host, ip);
        }
    }
    
    Ok(())
}

fn main() {
    let _ = p1("p1.txt");
    let _ = p2("p2.txt", "p2_criptat.txt");
    let _ = p3("p3.txt");
    let _ = p4();
}