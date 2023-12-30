fn factorial(n: u32, depth: u32) -> u32 {

    println!("iterasi {} : n = {}", depth, n);

    if n == 0 {
        return 1;
    } else {
        return n * factorial(n - 1, depth + 1);
    }
}

fn main() {
    let n = 1;
    let result = factorial(n, 1);

    println!("Bilangan factorial dari {} adalah {}", n, result);
}
