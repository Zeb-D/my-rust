mod ffi_text_analysis;

fn main() {
    let numbers = vec![0, 1, 2, 3, 4];
    let i = numbers.len() / 2;

    let x = *unsafe { numbers.get_unchecked(i) };
    println!("{x}")
}
