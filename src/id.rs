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

pub fn with2<R, F: for<'id1, 'id2> FnOnce(Id<'id1>, Id<'id2>) -> R>(func: F) -> R {
    with(|id1| {
        with(|id2| func(id1, id2))
    })
}
