// CECS 342 - Program 5: Heap Sort Visualizer
// Displays heap sort process with tree visualization during heap building
// and array output during sorting phase

use std::io::{self, Read};

const TREE_SIZE: usize = 31;
const MIN_VALUE: u32 = 10;
const MAX_VALUE: u32 = 99;

fn generate_random_unique_numbers() -> Vec<u32> {
    use std::collections::HashSet;
    let mut numbers = Vec::new();
    let mut used = HashSet::new();
    
    // Simple pseudo-random generator
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let mut rng = seed;
    while numbers.len() < TREE_SIZE {
        rng = (rng.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let num = ((rng % ((MAX_VALUE - MIN_VALUE + 1) as u64)) as u32) + MIN_VALUE;
        
        if !used.contains(&num) {
            used.insert(num);
            numbers.push(num);
        }
    }
    
    numbers
}

fn display_tree(arr: &[u32], size: usize) {
    if size == 0 {
        return;
    }

    let levels = 5; // We know it's size 31, so 5 levels (0-4)
    let mut index = 0;

    for level in 0..levels {
        let items_in_level = 1 << level;
        
        // Calculated identation (initial_padding) and gap (between_padding)
        // to ensure parents are perfectly centered above children.
        // L4 (Leaves): Indent 0, Gap 2
        // L3: Indent 2, Gap 6
        // L2: Indent 6, Gap 14
        // L1: Indent 14, Gap 30
        // L0: Indent 30
        let initial_padding = match level {
            0 => 30,
            1 => 14,
            2 => 6,
            3 => 2,
            4 => 0,
            _ => 1,
        };
        
        let between_padding = match level {
            0 => 0,
            1 => 30,
            2 => 14,
            3 => 6,
            4 => 2,
            _ => 1,
        };

        // Print initial padding
        print!("{:width$}", "", width = initial_padding);

        for i in 0..items_in_level {
            if index < size {
                print!("{:02}", arr[index]);
                index += 1;
            } else {
                break;
            }
            
            // Print padding between items, but not after the last one
            if i < items_in_level - 1 {
                print!("{:width$}", "", width = between_padding);
            }
        }
        println!("\n"); 
    }
}

fn display_array(arr: &[u32], size: usize) {
    print!("[");
    for i in 0..size {
        print!("{}", arr[i]);
        if i < size - 1 {
            print!(", ");
        }
    }
    println!("]");
}

fn heapify_up(arr: &mut [u32], index: usize) {
    if index == 0 {
        return;
    }
    
    let parent = (index - 1) / 2;
    if arr[index] > arr[parent] {
        arr.swap(index, parent);
        heapify_up(arr, parent);
    }
}

fn heapify_down(arr: &mut [u32], size: usize, index: usize) {
    let mut largest = index;
    let left = 2 * index + 1;
    let right = 2 * index + 2;
    
    if left < size && arr[left] > arr[largest] {
        largest = left;
    }
    
    if right < size && arr[right] > arr[largest] {
        largest = right;
    }
    
    if largest != index {
        arr.swap(index, largest);
        heapify_down(arr, size, largest);
    }
}

fn wait_for_enter() {
    let mut buffer = [0u8; 1];
    let _ = io::stdin().read(&mut buffer);
}

fn main() {
    let mut arr = generate_random_unique_numbers();
    
    println!("Unsorted binary tree:");
    display_tree(&arr, TREE_SIZE);
    
    println!("\nPress Enter to start the heap sort...");
    wait_for_enter();
    
    // Build max heap (heapify up from each node)
    for i in 1..TREE_SIZE {
        heapify_up(&mut arr, i);
    }
    
    // Sorting phase - extract max and heapify down
    for i in (1..TREE_SIZE).rev() {
        // Swap root with last element
        arr.swap(0, i);
        
        // Re-heapify the reduced heap
        heapify_down(&mut arr, i, 0);
        
        // Display the array after each extraction
        display_array(&arr, TREE_SIZE);
    }
    
    println!("\nSorted Array:");
    display_array(&arr, TREE_SIZE);
    
    println!("\nPress Enter to see the sorted tree...");
    wait_for_enter();
    
    display_tree(&arr, TREE_SIZE);
}
