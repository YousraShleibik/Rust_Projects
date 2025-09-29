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


#[derive(Debug, Clone, Copy)]
enum Material {
    Copper,
    Silver,
    Gold,
    Electrum,
    Mithril,
}

#[derive(Debug, Clone, Copy)]
enum Edge {
    Smooth,
    Reeded,
    Patterned,
}

#[derive(Debug, Clone, Copy)]
enum Size {
    Small,    // ~18mm
    Medium,   // ~20mm
    Standard, // ~22mm
    Large,    // ~28mm
}

impl Size {
    fn diameter_mm(self) -> u8 {
        match self {
            Size::Small => 18,
            Size::Medium => 20,     // pick any value you like
            Size::Standard => 22,
            Size::Large => 28,
        }
    }
}

#[derive(Debug, Clone)]
struct Coin {
    denom: Denomination,
    material: Material,
    size: Size,
    edge: Edge,
    year: u16,          // in-game mint year
    commemorative: bool,
}

impl Coin {
    /// Constructor (associated function on the struct)
    fn new(
        denom: Denomination,
        material: Material,
        size: Size,
        edge: Edge,
        year: u16,
        commemorative: bool,
    ) -> Self {
        Self { denom, material, size, edge, year, commemorative }
    }

    /// Method on the struct that *uses* an enum method.
    fn value_in_cp(&self) -> u32 {
        self.denom.to_copper_value()
    }
}

/// Pretty printing to show all fields clearly.
impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let material = match self.material {
            Material::Copper => "Copper",
            Material::Silver => "Silver",
            Material::Gold => "Gold",
            Material::Electrum => "Electrum",
            Material::Mithril => "Mithril",
        };
        let edge = match self.edge {
            Edge::Smooth => "Smooth",
            Edge::Reeded => "Reeded",
            Edge::Patterned => "Patterned",
        };
        let denom_name = match self.denom {
            Denomination::Copper => "Copper",
            Denomination::Silver => "Silver",
            Denomination::Gold => "Gold",
            Denomination::Platinum => "Platinum",
        };

        write!(
            f,
            "{year} {denom_name} — {material}, {diam}mm, {edge}{comm}",
            year = self.year,
            denom_name = denom_name,
            material = material,
            diam = self.size.diameter_mm(),
            edge = edge,
            comm = if self.commemorative { ", Commemorative" } else { "" }
        )
    }
}

/// Helper: convert a copper total to a readable string using denominations.
fn cp_to_pretty(mut cp: u32) -> String {
    let pp = cp / 1_000; cp %= 1_000;
    let gp = cp / 100;   cp %= 100;
    let sp = cp / 10;    cp %= 10;

    let mut parts = Vec::new();
    if pp > 0 { parts.push(format!("{pp} pp")); }
    if gp > 0 { parts.push(format!("{gp} gp")); }
    if sp > 0 { parts.push(format!("{sp} sp")); }
    if cp > 0 || parts.is_empty() { parts.push(format!("{cp} cp")); }
    parts.join(" + ")
}



fn main() {
    println!("== Golden Dog Game Company: Coin Demo (Structs + Enums) ==\n");

    // Create several coins to demonstrate different enum variants.
let copper_basic = Coin::new(
        Denomination::Copper,
        Material::Copper,
        Size::Small,
        Edge::Smooth,
        1325,
        false,
    );

    let silver_trade = Coin::new(
        Denomination::Silver,
        Material::Silver,
        Size::Standard,
        Edge::Reeded,
        1326,
        false,
    );

    let gold_guild = Coin::new(
        Denomination::Gold,
        Material::Gold,
        Size::Standard,
        Edge::Patterned,
        1327,
        true, // commemorative
    );

    let platinum_crown = Coin::new(
        Denomination::Platinum,
        Material::Mithril,
        Size::Large,
        Edge::Patterned,
        1328,
        true,
    );


    // Put them in a Vec and print properties + computed values.
    let coins = vec![copper_basic, silver_trade, gold_guild, platinum_crown];

        for (i, coin) in coins.iter().enumerate() {
        let value_cp = coin.value_in_cp();
        println!("Coin #{i}: {coin}");
        println!("  • Value: {} cp ({})", value_cp, cp_to_pretty(value_cp));
        println!("  • Symbol: {}", coin.denom.symbol());
        println!();
    }

    // Bonus: total value example (still using struct + enum methods)
    let total_cp: u32 = coins.iter().map(|c| c.value_in_cp()).sum();
    println!("Total purse value: {} cp ({})", total_cp, cp_to_pretty(total_cp));

    //let denom = Denomination::Gold;
    //println!("Value of {:?} is {} {}", denom, denom.to_copper_value(), denom.symbol());
}
