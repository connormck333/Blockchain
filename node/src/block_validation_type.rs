#[derive(PartialEq, Debug)]
pub enum BlockValidationType {
    Valid,
    Invalid,
    Fork
}