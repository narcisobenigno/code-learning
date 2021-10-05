
fn qsort(elements: &[i32]) -> Vec<i32> {
    if elements.is_empty() {
        return vec![];
    }

    let pivot = elements[0];
    let tail = elements[1..].to_vec();
    let smaller: Vec<i32> = tail.iter().cloned().filter(|x| *x <= pivot).collect();
    let bigger: Vec<i32> = tail.iter().cloned().filter(|x| *x > pivot).collect();

    [
        qsort(smaller.as_slice()),
        vec![pivot],
        qsort(bigger.as_slice()),
    ].concat()
}

#[cfg(test)]
mod test {
    use algorithms::qsort::qsort;

    #[test]
    fn it_sorts_elements() {
        let sorted = qsort(&[1000, 1, 10, 200, 20, 3000, 10000]);
        assert_eq!(
            vec![1, 10, 20, 200, 1000, 3000, 10000],
            sorted
        )
    }
}