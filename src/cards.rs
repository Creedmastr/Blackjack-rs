#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub name: &'static str,
    pub value: i8,
}