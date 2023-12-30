fn triangular_number(n: u32, depth: u32) -> u32 {
    println!("iterasi {} : n = {}", depth, n);

    if n == 1 {
        return 1;
    } else {
        return n + triangular_number(n - 1, depth + 1);
    }
}

fn main() {
    let n = 8;
    let result = triangular_number(n, 1);

    println!("Bilangan segitiga ke-{} adalah {}", n, result);
}
