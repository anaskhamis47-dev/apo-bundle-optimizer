use std::fs;
use std::io;
use serde::{Serialize, Deserialize};

// تعريف المنتج
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Product {
    name: String,
    category: String, // e.g., "Painkiller", "Vitamin", "Skin", "Hair"
    price: f64,
    days_to_expiry: u32,
}

fn main() {
    let mut inventory: Vec<Product> = load_data();

    loop {
        println!("\n=== Smart Pharmacy Manager (Passau) ===");
        println!("1. Add Product");
        println!("2. Show Inventory");
        println!("3. Generate Smart Bundles (AI Logic)");
        println!("4. Save & Exit");
        println!("Enter choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed");

        match choice.trim() {
            "1" => {
                let name = get_input("Product Name:");
                println!("(Categories: Painkiller, Vitamin, Skin, Hair)");
                let category = get_input("Category:");
                let price: f64 = get_input("Price (€):").parse().unwrap_or(0.0);
                let days: u32 = get_input("Days to Expiry:").parse().unwrap_or(365);

                inventory.push(Product {
                    name,
                    category, 
                    price,
                    days_to_expiry: days,
                });
                save_data(&inventory); 
                println!("Saved!");
            }
            "2" => {
                println!("\n--- Current Stock ---");
                for p in &inventory {
                    println!("- {} [{}]: {}€ (Expires in {} days)", p.name, p.category, p.price, p.days_to_expiry);
                }
            }
            "3" => {
                suggest_smart_bundles(&inventory);
            }
            "4" => {
                save_data(&inventory);
                println!("Data Saved. Auf Wiedersehen!");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}

// دالة مساعدة لتقليل الكتابة
fn get_input(text: &str) -> String {
    println!("{}", text);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    input.trim().to_string()
}

// === المخ الذكي (Smart Logic) ===
fn suggest_smart_bundles(inventory: &Vec<Product>) {
    println!("\nAnalyzing Inventory for Smart Matches...");

    // 1. فلترة المنتجات اللي قربت تخلص
    let expiring_items: Vec<&Product> = inventory.iter()
        .filter(|p| p.days_to_expiry < 90)
        .collect();

    if expiring_items.is_empty() {
        println!("Good news: No stock is at risk of expiring soon.");
        return;
    }

    // 2. تقسيم المنتجات لمجموعات
    let painkillers: Vec<&Product> = expiring_items.iter().filter(|p| p.category.to_lowercase().contains("pain")).map(|&p| p).collect();
    let vitamins: Vec<&Product> = expiring_items.iter().filter(|p| p.category.to_lowercase().contains("vit")).map(|&p| p).collect();
    let skin_care: Vec<&Product> = expiring_items.iter().filter(|p| p.category.to_lowercase().contains("skin")).map(|&p| p).collect();
    let hair_care: Vec<&Product> = expiring_items.iter().filter(|p| p.category.to_lowercase().contains("hair")).map(|&p| p).collect();

    let mut bundle_found = false;

    // === السيناريو الأول: باقة المناعة ===
    if !painkillers.is_empty() && !vitamins.is_empty() {
        println!("\n>>> 💡 SMART BUNDLE: 'Immunity Defense Kit'");
        let p = painkillers[0];
        let v = vitamins[0];
        
        let original = p.price + v.price;
        let discount = if p.days_to_expiry < 30 { 0.50 } else { 0.20 };
        
        println!("   Items: {} + {}", p.name, v.name);
        println!("   Logic: Combine Painkiller & Vitamin for flu season.");
        println!("   Price: {:.2}€ (was {:.2}€) - Discount: {}%", original * (1.0 - discount), original, discount * 100.0);
        bundle_found = true;
    }

    // === السيناريو الثاني: باقة الجمال ===
    if !skin_care.is_empty() && !hair_care.is_empty() {
        println!("\n>>> 💡 SMART BUNDLE: 'Self-Care Box'");
        let s = skin_care[0];
        let h = hair_care[0];

        let original = s.price + h.price;
        let discount = 0.30; 

        println!("   Items: {} + {}", s.name, h.name);
        println!("   Logic: Skin + Hair products together.");
        println!("   Price: {:.2}€ (was {:.2}€) - Discount: 30%", original * (1.0 - discount), original);
        bundle_found = true;
    }

    if !bundle_found {
        println!("No smart pairs found. Individual clearance items:");
        for p in expiring_items {
            let discount = if p.days_to_expiry < 30 { 0.50 } else { 0.20 };
            println!("- {} ({} days left): {:.2}€ (-{}%)", p.name, p.days_to_expiry, p.price * (1.0 - discount), discount * 100.0);
        }
    }
}

// دوال الحفظ والتحميل
fn save_data(inventory: &Vec<Product>) {
    let json = serde_json::to_string_pretty(inventory).expect("Error");
    fs::write("inventory.json", json).expect("Error");
}

fn load_data() -> Vec<Product> {
    match fs::read_to_string("inventory.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}
