fn hanoi(n: u32, from: char, to: char, aux: char) {
    if n == 1 {
        println!("Pindahkah cakram 1 dari {} ke {}", from, to);
    } else {
        hanoi(n - 1, from, aux, to);
        println!("Pindahkan cakram {} dari {} ke {}", n, from, to);
        hanoi(n - 1, aux, to, from);
    }
}

fn main() {
    let n = 2;
    hanoi(n, 'A', 'C', 'B');
}
