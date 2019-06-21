use crate::attack_target::AttackTarget;

pub trait Attacker {
    fn attack<T: AttackTarget>(&mut self, target: &mut T);
}
