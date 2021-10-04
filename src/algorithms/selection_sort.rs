fn selection_sort(elements: &[i32]) -> Vec<i32> {
    selection_sort_aux(elements, &mut Vec::with_capacity(elements.len()))
}

fn selection_sort_aux(elements: &[i32], sorted: &mut Vec<i32>) -> Vec<i32> {
    if elements.is_empty() {
        return sorted.to_vec();
    }
    let i = find_smallest(elements);
    sorted.push(elements[i]);

    let remaining = [&elements[..i], &elements[i+1..]].concat();
    selection_sort_aux(&remaining, sorted)
}

fn find_smallest(elements: &[i32]) -> usize {
    let mut smallest = elements[0];
    let mut index = 0;
    for i in 1..elements.len() {
        if smallest > elements[i] {
            index = i;
            smallest = elements[i];
        }
    }
    return index;
}

#[cfg(test)]
mod test {
    use algorithms::selection_sort::selection_sort;

    #[test]
    fn it_sorts_array() {
        let sorted = selection_sort(&[1000, 1, 10, 200, 20, 3000, 10000]);
        assert_eq!(
            vec![1, 10, 20, 200, 1000, 3000, 10000],
            sorted
        )
    }
}