fn insertion_sort(array: &mut [i8]) {
    for i in 0..array.len() {
        for j in (0..i).rev() {
            if array[j] >= array[j + 1] {
                array.swap(j, j + 1);
            } else {
                break;
            }
        }
    }
}

fn main() {
    println!("___ Insertion Sort ___");

    let mut num_list = [ 5, 9, 8, 4, 1, 10, 7, 2, 3, 6, 0 ];
    println!("unsorted list: {:?}", num_list);

    insertion_sort(&mut num_list);

    println!("sorted list: {:?}", num_list);
}