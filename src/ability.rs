pub trait AbilityTarget {}

pub trait Ability {
    type Target: AbilityTarget;

    fn resolve(target: &mut Self::Target);
}

pub mod deathtouch;
