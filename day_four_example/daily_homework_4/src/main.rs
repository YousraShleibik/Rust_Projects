use std::fmt;

/// Represents the different denominations of currency in a role-playing game.
#[derive(Debug, Clone, Copy)]
enum Denomination {
    Copper,   // 1 cp
    Silver,   // 10 cp
    Gold,     // 100 cp
    Platinum, // 1000 cp
}


impl Denomination {
    /// Associated function on the enum: map variant -> base value (in copper pieces).
    fn to_copper_value(self) -> u32 {
        match self {
            Denomination::Copper => 1,
            Denomination::Silver => 10,
            Denomination::Gold => 100,
            Denomination::Platinum => 1_000,
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            Denomination::Copper => "cp",
            Denomination::Silver => "sp",
            Denomination::Gold => "gp",
            Denomination::Platinum => "pp",
        }
    }
}

fn main() {
    let denom = Denomination::Gold;
    println!("Value of {:?} is {} {}", denom, denom.to_copper_value(), denom.symbol());
}
