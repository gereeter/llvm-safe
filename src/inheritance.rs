use std::mem::transmute_copy;

pub unsafe trait DerivesFrom<Ancestor: ?Sized> { }
pub unsafe trait DerivesFromSized<Ancestor>: DerivesFrom<Ancestor> { }

pub fn upcast<'a, Specific: DerivesFrom<General> + ?Sized, General: ?Sized>(object: &'a Specific) -> &'a General {
    unsafe {
        &*(transmute_copy::<_, *const General>(&(object as *const Specific)))
    }
}

pub unsafe fn downcast_unchecked<'a, Specific: DerivesFrom<General> + ?Sized, General: ?Sized>(object: &'a General) -> &'a Specific {
    &*(transmute_copy::<_, *const Specific>(&(object as *const General)))
}

unsafe impl<'a, Specific: DerivesFrom<General>, General> DerivesFrom<&'a General> for &'a Specific { }
unsafe impl<'a, Specific: DerivesFrom<General>, General> DerivesFromSized<&'a General> for &'a Specific { }
unsafe impl<Specific: DerivesFromSized<General>, General> DerivesFrom<[General]> for [Specific] { }
