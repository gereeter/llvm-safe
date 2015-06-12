use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct IdRef<'id> {
    _marker: PhantomData<(&'id (), fn(&'id ()))>
}

impl<'id> IdRef<'id> {
    pub fn new() -> IdRef<'id> {
        IdRef { _marker: PhantomData }
    }
}

pub struct Id<'id> {
    _marker: IdRef<'id>
}

pub fn with<R, F: for<'id> FnOnce(Id<'id>) -> R>(func: F) -> R {
    func(Id { _marker: IdRef::new() })
}
