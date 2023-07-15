use rbuwu::{eprintwn, epwint, pwint, pwintwn};

fn main() {
    let hewwo = "Hewwo Wowwd";
    pwintwn!("{}", hewwo);
    pwint!("One Line");
    let hello = "Hello";
    pwintwn!("{}", hello);
    epwint!("epwint error");
    eprintwn!("Error");
}
