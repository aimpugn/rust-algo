pub trait Algo<T> {
    fn doit(&self) -> Option<T>;
}