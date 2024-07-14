// in the future, using `match` instead of a closure
// will allow them to be `const`

pub fn unwrapable_o<T>(x: Option<T>) -> T {
    x.unwrap_or_else(|| unreachable!())
}

pub fn unwrapable_r<T, E>(x: Result<T, E>) -> T {
    x.unwrap_or_else(|_| unreachable!())
}
