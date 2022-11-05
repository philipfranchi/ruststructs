mod tests;

pub fn search<T: PartialEq + PartialOrd>(v: &Vec<T>, target: T) -> bool {
    let len = v.len();
    if len == 0 {return false;}
    _recurse(&v, &target, 0, len - 1)
}

fn _recurse<T: PartialEq + PartialOrd>(v: &Vec<T>, target: &T, min: usize, max: usize) -> bool {
    // pick a spot in the middle
    if max == min {
        return  v[max] == *target;
    }
    
    let mid = (max + min)/2;
    let val = &v[mid];
    if val == target {
        return true
    }
    else if val > target {
        return _recurse(v, target, min, mid)
    } else {
        return _recurse(v, target, mid + 1, max)
    }
}
