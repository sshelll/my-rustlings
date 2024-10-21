fn fill_vec(vec: &[i32]) -> Vec<i32> {
    let mut vec = vec.to_owned();

    vec.push(88);

    vec
}

fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(&vec0);

    assert_eq!(vec0, [22, 44, 66]);
    assert_eq!(vec1, [22, 44, 66, 88]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(&vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
