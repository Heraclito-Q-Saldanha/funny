trait Bar<U> {
    type Output;
}

impl<T: ?Sized, U> Bar<U> for T {
    type Output = U;
}

fn foo<T: ?Sized, U>(x: <T as Bar<U>>::Output) -> U {
    x
}

#[inline(always)]
pub fn transmute<U, T>(x: T) -> U {
    foo::<dyn Bar<U, Output = T>, U>(x)
}
