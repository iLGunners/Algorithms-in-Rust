fn bubble_sort(array: &mut[i8]) {
    let mut array_length = array.len();
    let mut swapped_flag = true;
 
    while swapped_flag {
        swapped_flag = false;
 
        for i in 1..array_length {
            if array[i - 1] > array[i] {
                array.swap(i - 1, i);
                swapped_flag = true;
            }
        }
 
        array_length = array_length - 1;
    }
}

fn main() {
    println!("___ Bubble Sort ___");

    let mut num_list = [ 5, 9, 8, 4, 1, 10, 7, 2, 3, 6, 0 ];
    println!("unsorted list: {:?}", num_list);

    bubble_sort(&mut num_list);

    println!("sorted list: {:?}", num_list);
}
