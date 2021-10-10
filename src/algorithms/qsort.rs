use rand::Rng;

pub fn qsort(elements: &[i32]) -> Vec<i32> {
    if elements.is_empty() {
        return vec![];
    }

    let pivot_i = rand::thread_rng().gen_range(0..elements.len());
    let pivot = elements[pivot_i];
    let mut smaller: Vec<i32> = Vec::with_capacity(elements.len());
    let mut bigger: Vec<i32> = Vec::with_capacity(elements.len());

    for (i, e) in elements.iter().enumerate() {
        if pivot_i == i { continue }

        if *e < pivot { smaller.push(*e) }
        else { bigger.push(*e) }
    }

    [
        qsort(&smaller[0..]),
        vec![pivot],
        qsort(&bigger[0..]),
    ].concat()
}

#[cfg(test)]
mod test {
    use crate::algorithms::qsort;

    #[test]
    fn it_sorts_elements() {
        let sorted = qsort(&[1000, 1, 10, 200, 20, 3000, 10000]);
        assert_eq!(
            vec![1, 10, 20, 200, 1000, 3000, 10000],
            sorted
        )
    }
}