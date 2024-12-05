fn main() {
    let mut v = vec![1, 2, 3];
    let mut v_clone = v.clone();
    let ptr = v_clone.as_mut_ptr();
    unsafe {
        *ptr = 10;
    }
    println!("v: {:?}", v_clone);
    println!("v original: {:?}", v);
}