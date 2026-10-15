fn main() {
    let mut array: [i16; 8] = [-1; 8];

    array[0] = 100;
    array[4] = 250;
    array[7] = array[0] + array[4];

    println!("{:?}", array);
}
