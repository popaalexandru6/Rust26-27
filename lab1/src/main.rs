fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i = i + 1;
    }
    return true;
}

fn cmmdc(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let aux = b;
        b = a % b;
        a = aux;
    }
    return a;
}

fn is_coprime(a: u32, b: u32) -> bool {
    if cmmdc(a, b) == 1 {
        return true;
    }
    return false;
}

fn main() {
    println!("Numere prime: \n");
    for a in 0..100 {
        if is_prime(a) {
            println!("{}", a);
        }
    }
    println!("Numere coprime:\n");

    for a in 0..100 {
        for b in a..100 {
            if is_coprime(a, b) {
                println!("{}, {}", a, b);
            }
        }
    }
    println!("99 bottles of beer:\n");
    for i in (2..=99).rev() {
        println!("{} bottles of beer on the wall,", i);
        println!("{} bottles of beer.", i);
        println!("Take one down, pass it around,");
        println!("1 bottle of beer on the wall,");
        println!("1 bottle of beer.");
        println!("Take one down, pass it around,");
    }
}
