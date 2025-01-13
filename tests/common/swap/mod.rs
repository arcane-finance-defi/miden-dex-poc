pub fn calculate_swap(reserve_in: u32, reserve_out: u32, amount_in: u32) -> u32 {
    let scaled_reserve_in: u64 = (reserve_in * 1000) as u64;
    let scaled_reserve_out: u64 = (reserve_out * 1000) as u64;
    let scaled_amount_in: u64 = (amount_in * 1000) as u64;

    let numerator: u32 = ((scaled_amount_in * scaled_reserve_out) / 1000000) as u32;
    let denominator: u32 = ((scaled_reserve_in + scaled_amount_in) / 1000) as u32;
    let amount_out = numerator / denominator;
    amount_out
}