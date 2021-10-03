
pub fn binary_search(elements: Vec<i32>, element: i32) -> Result<usize, String> {
    let high = elements.len() -1;
    binary_search_aux(elements, element, 0, high)
}

fn binary_search_aux(elements: Vec<i32>, element: i32, low: usize, high: usize) -> Result<usize, String> {
    if low > high {
        return Err("not found".to_string())
    }

    return match ((high + low) / 2, element) {
        (mid, element) if elements[mid] < element => binary_search_aux(elements, element, mid + 1, high),
        (mid, element) if elements[mid] > element => binary_search_aux(elements, element, low, mid - 1),
        (mid, _) => Ok(mid),
    }
}

#[cfg(test)]
mod test {
    use algorithms::binary_search::binary_search;

    #[test]
    fn it_finds_element_position() {
        let found = binary_search(vec![1, 3, 5, 33, 55, 99, 888, 1000, 1001, 1002, 1003], 888);
        assert_eq!(Ok(6usize), found)
    }

    #[test]
    fn it_does_not_find_element_position() {
        let found = binary_search(vec![1, 3, 5, 33, 55, 99, 888, 1000, 1001, 1002, 1003], 999);
        assert_eq!(Err("not found".to_string()), found)
    }
}