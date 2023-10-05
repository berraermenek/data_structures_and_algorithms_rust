fn main() {
    let mut heap = Heap::new();
    insert(&mut heap, 31);
    insert(&mut heap, 21);
    insert(&mut heap, 48);
    insert(&mut heap, 9);
    delete_min(&mut heap);
    insert(&mut heap, 6);
    insert(&mut heap, 5);
    insert(&mut heap, 2);
    insert(&mut heap, 3);
    insert(&mut heap, 1);
    delete_min(&mut heap);
    delete_min(&mut heap);
}

#[derive(Debug)]
struct Heap {
    data: Vec<usize>,
    last: usize,
}
impl Heap {
    fn new() -> Self {
        Heap {
            data: Vec::new(),
            last: 0,
        }
    }
}

fn print_heap(data: &[usize], index: usize, depth: usize, prefix: &str, is_left: bool) {
    if index < data.len() {
        let next_prefix = if is_left { "     " } else { " |   " };
        print_heap(
            data,
            2 * index + 2,
            depth + 1,
            &format!("{}{}", prefix, next_prefix),
            false,
        );

        println!(
            "{}{}{}{}",
            prefix,
            if is_left { " \\-- " } else { " /-- " },
            data[index],
            if 2 * index + 2 < data.len() { " " } else { "" }
        );

        print_heap(
            data,
            2 * index + 1,
            depth + 1,
            &format!("{}{}", prefix, next_prefix),
            true,
        );
    }
}

fn insert(heap: &mut Heap, object: usize) {
    heap.data.push(0);
    heap.last += 1;
    heap.data[heap.last - 1] = object;

    // 入れる値のindex
    let mut i = heap.last - 1;
    while i > 0 {
        let parent_index = i / 2;
        let current_index = i;
        let parent_data = heap.data[parent_index];
        let current_data = heap.data[current_index];

        if parent_data > current_data {
            heap.data.swap(parent_index, current_index);
            i = parent_index;
        } else {
            break;
        }
    }
    println!("inserted: {:?}", heap.data);
    print_heap(&heap.data, 0, 0, "", false);
    println!();
}

fn delete_min(heap: &mut Heap) -> usize {
    let object = heap.data[0];
    let swap_data = heap.data[heap.last - 1];
    heap.data[0] = swap_data;
    heap.last -= 1;
    heap.data.pop();
    let mut swap_data_index = 0;
    while 2 * swap_data_index + 1 < heap.last {
        let left_child_index = 2 * swap_data_index + 1;
        let right_child_index = 2 * swap_data_index + 2;
        let left_child = heap.data[left_child_index];
        // 右の子が存在する場合はその値を、存在しない場合はusize::MAXを使用します。
        let right_child = if right_child_index < heap.last {
            heap.data[right_child_index]
        } else {
            usize::MAX
        };

        // 右の子が存在し、左の子よりも小さい場合、min_childを更新します。
        if swap_data > left_child {
            heap.data.swap(swap_data_index, left_child_index);
            swap_data_index = left_child_index;
        } else if swap_data > right_child {
            heap.data.swap(swap_data_index, right_child_index);
            swap_data_index = right_child_index;
        } else {
            break;
        }
    }
    println!("deleted: {:?}", heap.data);
    print_heap(&heap.data, 0, 0, "", false);
    println!();
    object
}
