// This works
// mod generated {
//     include!(concat!(env!("OUT_DIR"), "/generated.rs"));
// }


// While this doesn't
#[path = concat!(env!("OUT_DIR"), "/generated.rs")]
mod generated;


pub fn blackjack() {
    generated::bar();
}
