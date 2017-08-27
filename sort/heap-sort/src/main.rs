// heapify a subtree rooted with node index, which is an index in array[]
// length is size of heap
fn heapify(array: &mut [i8], length: usize, index: usize) {
    let mut largest = index; // Initialize largest as root
    let left = 2*index + 1;
    let right = 2*index + 2;

    // if left child is larger than root
    if left < length && array[left] > array[largest] {
        largest = left;
    }

    // if right child is larger than root
    if right < length && array[right] > array[largest] {
        largest = right;
    }

    // if largest is not root
    if largest != index {
        array.swap(index, largest);
        // recursively heapify affected sub-tree
        heapify(array, length, largest);
    }
}

// function to do heap sort
fn heap_sort(array: &mut [i8]) {
    let array_length = array.len();
    
    // build heap
    for i in (0..array_length/2+1).rev() {
        heapify(array, array_length, i);
    }

    // extract element from heap
    for i in (1..array_length).rev() {
        // move current root to end
        array.swap(0, i);
        // heapify reduced heap
        heapify(array, i-1, 0);
    }
}

// driver function
fn main() {
    println!("___ Heap Sort ___");

    let mut num_list = [ 5, 9, 8, 4, 1, 10, 7, 2, 3, 6, 0 ];
    println!("unsorted list: {:?}", num_list);

    heap_sort(&mut num_list);

    println!("sorted list: {:?}", num_list);
}
