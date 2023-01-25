fn is_ok<T: Ord>(index: usize, key: T, arr: &Vec<T>) -> bool {
    if arr[index] >= key {
        true
    } else {
        false
    }
}

fn binary_search<T: Ord + Copy>(key: T, arr: Vec<T>) -> i32 {
    let mut ng = -1;
    let mut ok = arr.len() as i32;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid as usize, key, &arr) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

fn main() {
    let arr = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];
    println!("{}", binary_search(51, arr));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let arr = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];
        let act = binary_search(1, arr);
        let exp = 0;
        assert_eq!(act, exp);
    }
    #[test]
    fn case2() {
        let arr = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];
        let act = binary_search(910, arr);
        let exp = 9;
        assert_eq!(act, exp);
    }
    #[test]
    fn case3() {
        let arr = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];
        let act = binary_search(52, arr);
        let exp = 6;
        assert_eq!(act, exp);
    }
}
