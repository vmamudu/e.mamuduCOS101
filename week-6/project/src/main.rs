use std::io;

fn main() {
    // Display menu
    println!("\n ⋆˚꩜｡𐔌՞. 𝖳𝗁𝖾 𝖪𝗋𝗂𝗌𝗍𝖺𝗅𝖾𝗋𝗂𝖾 𝗋𝖾𝗌𝗍𝖺𝗎𝗋𝖺𝗇𝗍 .՞𐦯⋆. 𐙚 ˚");

    println!("\n-------> 𝗐𝗁𝖺𝗍 𝗐𝗈𝗎𝗅𝖽 𝗒𝗈𝗎 𝗅𝗂𝗄𝖾 𝗍𝗈 𝖾𝖺𝗍 𝗍𝗈𝖽𝖺𝗒 ?\nᴘ..." );
    println!("‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎‿̩͙");
    println!(" ﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌");
    println!("   [code]| Food Item                             | Price ");
    println!("\n ﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌");
    println!("\n ᯓ★ [pe] Poundo Yam / Edinkaiko Soup               | ₦3, 200
        \n ᯓ★ [fc] Fried Rice & Chicken                      | ₦3, 000
        \n ᯓ★ [ae] Amala & Ewedu Soup                        | ₦2, 500
        \n ᯓ★ [ee] Eba & Egusi Soup                          | ₦2, 000
        \n ᯓ★ [ws] White Rice & Stew                         | ₦2, 500");
    println!(" ﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌﹌");
    println!("⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎༻♱༺༒︎⊰‿̩͙‿̩͙⊱༒︎༻♱༺༒︎");

    // First item order
    println!("ɪɴᴘᴜᴛ ꜰᴏᴏᴅ ᴄᴏᴅᴇ: ");
    let mut order = String::new();
    io::stdin().read_line(&mut order).expect("Not a valid food code!! :-(");
    let order = order.trim().to_lowercase();
    
    let price = match order.as_str() {
        "pe" => 3_200.0,
        "fc" => 3_000.0,
        "ae" => 2_500.0,
        "ee" => 2_000.0,
        "ws" => 2_500.0,
        _ => {
            println!("invalid code !!");
            return;
        }
    };

    println!(" ɴᴜᴍʙᴇʀ ᴏf ᴘᴏʀᴛɪᴏɴꜱ: ");
    let mut portions = String::new();
    io::stdin().read_line(&mut portions).expect("not a valid number.");
    let portions: f32 = portions.trim().parse().expect("not a valid input :-(");

    
    println!("Would you like to order anything else? (yes/no)");
    let mut neworder = String::new();
    io::stdin().read_line(&mut neworder).expect("invalid input :(");
    
    
    let choice = match neworder.trim().to_lowercase().as_str() {
        "yes" => true,
        "no" => false,
        _ => {
            println!("invalid input :(");
            false
        }
    };

    let total: f32;

    if choice {
        println!("ɪɴᴘᴜᴛ ꜰᴏᴏᴅ ᴄᴏᴅᴇ: ");
        let mut order2 = String::new();
        io::stdin().read_line(&mut order2).expect("Not a valid food code!! :-(");
        let order2 = order2.trim().to_lowercase();
        
        let price2 = match order2.as_str() {
            "pe" => 3_200.0,
            "fc" => 3_000.0,
            "ae" => 2_500.0,
            "ee" => 2_000.0,
            "ws" => 2_500.0,
            _ => {
                println!("invalid code !!");
                return;
            }
        };

        println!("ɴᴜᴍʙᴇʀ ᴏf ᴘᴏʀᴛɪᴏɴꜱ: ");
        let mut portions2 = String::new();
        io::stdin().read_line(&mut portions2).expect("not a valid number.");
        let portions2: f32 = portions2.trim().parse().expect("not a valid input :-(");

        
        total = (price * portions) + (price2 * portions2);
    } else {
        total = price * portions;
    }


    if total >= 10000.0 {
        let discount = total * 0.05;
        let newtotal = total - discount;
        println!("\n𝖢𝗈𝗇𝗀𝗋𝖺𝗍𝗎𝗅𝖺𝗍𝗂𝗈𝗇𝗌 !!
            \n𝗒𝗈𝗎 𝗊𝗎𝖺𝗅𝗂𝖿𝗒 𝖿𝗈𝗋 𝖺 𝖽𝗂𝗌𝖼𝗈𝗎𝗇𝗍 ~ (𝗒𝖺𝗒𝗒𝗒)
            \n𝗒𝗈𝗎𝗋 *discounted* 𝗍𝗈𝗍𝖺𝗅 𝗂𝗌: ₦{}", newtotal);
    } else {
        println!("ʏᴏᴜʀ ᴛᴏᴛᴀʟ ɪꜱ : ₦{}", total);
    }
}
