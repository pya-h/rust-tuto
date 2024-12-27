use std::ops::Add;


struct Variable<T> {
    value: T
}

impl<T> Variable<T> {
    fn new(value: T) -> Self { Variable {value} }

    fn value(&self) -> &T {
        &self.value
    }
}

trait Addable<T> {
    fn add<W>(&self, value: &T) -> Variable<W>;
}

impl<T, U> Addable<U> for T where T: Copy + Add<Output=T> {
    fn add<W>(&self, value: &U) -> Variable<W> {
        Variable::new(*self + value)
    }
}

impl<T, U> Addable<Variable<U>> for T where T: Copy + Add<Output=T> {
    fn add<W>(&self, value: &Variable<U>) -> Variable<W> {
        Variable::new(*self + value.value())
    }
}

impl<T, U> Addable<T> for Variable<U> {
    fn add<W>(&self, value: &T) -> Variable<W> {
        todo!()
    }
}
/*
impl<T> Addable<T> for String {
    fn add(&self, value: T) -> Variable<T> {
        Variable::new(format!("{}{}", self, value))
    }
}*/
/*
impl<T> Variable<T> {
    fn add<U: Addable<T>, W>(&self, other: &U) -> Variable<W> {
        other.add(self)
    }
}
*/


fn main() {

}
