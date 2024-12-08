fn main() {
    let mut array = [19090,1265,1332,1818,1929, 2400];
    println!("Before sorting: {:?}", array);

    bubble_sort(&mut array);
    println!("After sorting: {:?}", array);
}

fn bubble_sort(array: &mut [i32]) {
    let len = array.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}
