use crate::attack_target::AttackTarget;

pub trait Attackable {
    fn attack<T: AttackTarget>(&mut self, target: &mut T);
}
