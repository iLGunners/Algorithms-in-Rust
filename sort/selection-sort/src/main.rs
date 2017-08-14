fn selection_sort(array: &mut [i32]) {
    for i in 0..array.len() {
        let min_index = (i..array.len()).min_by_key(|x| array[*x]).unwrap();
        array.swap(min_index, i)
    }
}

fn main() {
    println!("___ Selection Sort ___");

    let mut num_list = [ 5, 9, 8, 4, 1, 10, 7, 2, 3, 6, 0 ];
    println!("unsorted list: {:?}", num_list);

    selection_sort(&mut num_list);

    println!("sorted list: {:?}", num_list);
}
