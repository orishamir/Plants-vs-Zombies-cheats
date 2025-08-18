#![allow(dead_code)]
enum ArmorType {
    None,
    Cone,
    Bucket,
}

impl Into<u32> for ArmorType {
    fn into(self) -> u32 {
        match self {
            ArmorType::None => 0,
            ArmorType::Cone => 1,
            ArmorType::Bucket => 2,
        }
    }
}
pub struct Zombie {}
