pub trait AttackTarget {
    fn damage(&mut self, value: i32) -> i32;
    fn is_dead(&self) -> bool;
}
