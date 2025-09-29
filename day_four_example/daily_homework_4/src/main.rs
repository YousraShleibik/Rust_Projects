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

/// Material shows another enum; no behavior, just data categories.
#[derive(Debug, Clone, Copy)]
enum Material {
    Copper,
    Silver,
    Gold,
    Electrum,
    Mithril,
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
        Size::Medium,
        Edge::Reeded,
        1400,
        true,
    );
    let gold_guild = Coin::new(
        Denomination::Gold,
        Material::Gold,
        Size::Large,
        Edge::Decorated,
        1450,
        true,
    );
    let platinum_crown = Coin::new(
        Denomination::Platinum,
        Material::Mithril,
        Size::Large,
        Edge::Decorated,
        1500,
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
