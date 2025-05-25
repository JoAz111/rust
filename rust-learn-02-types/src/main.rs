use rand::Rng;

fn main() {
    //RuneGold Project
    let mut gold: i32 = 0;
    let mut total_earned: i64 = 0;

    for i in 1..=10 {
        let loot = rand::thread_rng().gen_range(50_000_000..=300_000_000);
        total_earned += loot;
        if gold + loot > 2_147_483_647 {
            println!("Iteration {}: Loot of {} exceeds i32 limit, upgrade to i64", i, loot);
            gold = i64::MAX as i32; // Upgrade to i64
            gold += loot;            
        }
        else {
            gold += loot as i32;
        }
        
    }
    println!("Total gold in pouch: {}", gold);
    println!("Total gold earned: {}", total_earned);

}
