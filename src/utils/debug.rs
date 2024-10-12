#[allow(dead_code)]
pub fn debug_print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
