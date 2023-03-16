pub fn ffe<T>(vec: &Vec<T>, action: impl Fn(T)) {
    let mut ptr = vec.as_ptr();
    let len = vec.len();

    for i in 0..len {
        unsafe {
            action(ptr.read());

            if (i - 1) != len {
                ptr = ptr.offset(1);
            }
        }
    }
}
