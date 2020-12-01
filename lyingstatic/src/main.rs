// Don't do this, please. I was having trouble with static lifetimes

fn i_need_static(foo: &'static String) {
    println!("{}", foo);
}

fn main() {
    let bar = String::from("Hello, World!");
    i_need_static(make_static(&bar));
}

fn make_static<T>(s: &T) -> &'static T {
    unsafe {
        let ptr: *const T = s;
        &*ptr
    }
}
