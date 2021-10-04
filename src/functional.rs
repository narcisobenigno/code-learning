pub trait Function<Out: Clone> {
    fn apply(&self) -> Out;
}

#[derive(Debug, Clone)]
pub struct Identity<X: Clone> {
    x: X,
}

impl<X: Clone> Identity<X> {
    pub fn new(x: X) -> Identity<X> {
        Self { x }
    }
}

impl<X: Clone> Function<X> for Identity<X> {
    fn apply(&self) -> X {
        self.x.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
