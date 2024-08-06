pub fn remove_trailing_elements<T: Eq + PartialEq>(v: &mut Vec<T>, x: T) {
    while let Some(tmp) = v.last() {
        if *tmp == x {
            v.pop();
        } else {
            break;
        }
    }
}
