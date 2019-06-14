use crate::ability::AbilityTarget;

pub trait Permanent: AbilityTarget {
    fn destroy(&mut self);
}
