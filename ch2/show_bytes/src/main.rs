use std::mem;

type ByteSlice<'a> = &'a[u8];

fn show_bytes(start: ByteSlice, len: usize) {
    for i in 0..len {
        print!(" {:x}", start[i])
    }
    println!()
}

fn show_int(x: i32) {
    show_bytes(&x.to_ne_bytes(), mem::size_of::<i32>());
}

fn show_float(x: f32) {
    show_bytes(&x.to_ne_bytes(), mem::size_of::<f32>());
}

fn show_pointer(x: *const i32) {
    show_bytes(&(x as usize).to_ne_bytes(), mem::size_of::<usize>());
}

fn test_show_bytes(val: i32) {
    let ival = val;
    let fval = val as f32;
    show_int(ival);
    show_float(fval);
    show_pointer((&ival) as *const i32);
}

fn main() {
    test_show_bytes(12345);
}
