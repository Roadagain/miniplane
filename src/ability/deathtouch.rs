use crate::ability::Ability;
use crate::creature::Creature;
use crate::permanent::Permanent;

pub struct Deathtouch {}

impl Ability for Deathtouch {
    type Target = Creature;

    fn resolve(target: &mut Self::Target) {
        target.destroy();
    }
}

#[cfg(test)]
mod test {
    use crate::ability::deathtouch::Deathtouch;
    use crate::ability::Ability;
    use crate::attack_target::AttackTarget;
    use crate::creature::Creature;

    #[test]
    fn death_by_deathtouch() {
        let mut creature = Creature::new(2, 2);
        Deathtouch::resolve(&mut creature);
        assert!(creature.is_dead());
    }
}
