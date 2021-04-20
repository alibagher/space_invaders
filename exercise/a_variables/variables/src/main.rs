const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {

    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    // let ready: i32 = READY_AMOUNT;
    READY_AMOUNT = 1;


    println!("Firing {} of my {} missiles...", ready, missiles);
    
    let _missilesss = 3;
    println!("{} missiles left", missiles - ready);
}
