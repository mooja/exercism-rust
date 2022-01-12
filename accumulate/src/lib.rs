/// What should the type of _function be?
pub fn map<T, U, F: FnMut(T) -> U >(input: Vec<T>, mut func: F) -> Vec<U>  {
    let mut rv: Vec<U> = Vec::new();
    for val in input.into_iter(){
        rv.push(func(val));
    }
    rv
}