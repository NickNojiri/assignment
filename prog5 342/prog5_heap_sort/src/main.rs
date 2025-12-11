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

    // Helper to get padding for a level: (initial, between)
    let get_padding = |level: usize| -> (usize, usize) {
        match level {
            0 => (30, 0),
            1 => (14, 30),
            2 => (6, 14),
            3 => (2, 6),
            4 => (0, 2),
            _ => (1, 1),
        }
    };

    // Helper to calculate centers for a level
    let get_centers = |level: usize, count: usize| -> Vec<usize> {
        let (initial, between) = get_padding(level);
        let mut centers = Vec::with_capacity(count);
        let mut current_pos = initial; 
        for _ in 0..count {
            // Number width is 2. Center is start + 1 (0-indexed relative to start)
            centers.push(current_pos + 1);
            current_pos += 2 + between;
        }
        centers
    };

    for level in 0..levels {
        let items_in_level = 1 << level;
        let (initial_padding, between_padding) = get_padding(level);

        // Print numbers
        print!("{:width$}", "", width = initial_padding);
        for i in 0..items_in_level {
            if index < size {
                print!("{:02}", arr[index]);
                index += 1;
            } else {
                break;
            }
            if i < items_in_level - 1 {
                print!("{:width$}", "", width = between_padding);
            }
        }
        println!();

        // Check if we need connectors (if not last level)
        // And if there are actually children to connect to
        if level < levels - 1 && index < size {
            let next_level = level + 1;
            let next_count = 1 << next_level;
            
            let current_centers = get_centers(level, items_in_level);
            let next_centers = get_centers(next_level, next_count);
            
            // Build the three connector lines
            // Maximum width logic: L4 is ~62 chars. 100 is safe.
            let max_width = 100;
            let mut row1 = vec![' '; max_width]; // Bars under parents
            let mut row2 = vec![' '; max_width]; // Horizontal dashes
            let mut row3 = vec![' '; max_width]; // Bars above children

            // Iterate over next level nodes
            let start_index_next = (1 << next_level) - 1;
            
            for i in 0..next_count {
                let child_idx = start_index_next + i;
                if child_idx >= size { break; }

                let child_center = next_centers[i];
                let parent_local_idx = i / 2;
                let parent_center = current_centers[parent_local_idx];

                // R3: Bar above child
                if child_center < max_width { row3[child_center] = '|'; }
                
                // R1: Bar below parent
                if parent_center < max_width { row1[parent_center] = '|'; }

                // R2: Horizontal dash
                let (start, end) = if child_center < parent_center {
                    (child_center + 1, parent_center) // LEFT child: dash rightwards to parent
                } else {
                    (parent_center + 1, child_center) // RIGHT child: dash rightwards from parent
                };
                
                for k in start..end {
                    if k < max_width { row2[k] = '-'; }
                }
                // Optional: connect exact intersection points?
                // Standard ASCII often puts corner pieces, but simple '-' is requested.
                // We leave the intersection at parent_center as ' ' or keep existing?
                // Actually, if we just fill start..end with '-', the parent center itself is NOT filled.
                // Which is good if we want "   |   " then "---|---"
                // But row2[parent_center] needs to be something?
                // If we want a continuous line:
                // Left child draws up to parent_center - 1.
                // Right child draws from parent_center + 1.
                // The parent center spot in Row 2: usually dashes go THROUGH it.
                // Let's force a dash at parent_center if it's being connected.
                if parent_center < max_width { row2[parent_center] = '-'; }
            }

            // Print the rows, trimming spaces
            let r1: String = row1.into_iter().collect();
            let r2: String = row2.into_iter().collect();
            let r3: String = row3.into_iter().collect();
            
            println!("{}", r1.trim_end());
            println!("{}", r2.trim_end());
            println!("{}", r3.trim_end());
        }
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

///code from nick nojiri's big brain
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
