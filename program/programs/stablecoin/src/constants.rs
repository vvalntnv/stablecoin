pub const CONFIG_SEED: &[u8] = b"config";
pub const COLLAT_SEED: &[u8] = b"collateral";
pub const MINT_SEED: &[u8] = b"mint";
pub const RESERVE_ACCOUNT_SEED: &[u8] = b"reserve-account";

pub const LIQUIDATION_THRESHOLD: u8 = 50;
pub const LIQUIDATION_BONUS: u8 = 10;

// If amount minted passes 80% of the collateral debited (collateral changes price),
// then the collateral + (collateral * liq_bonus) are sold to the market
pub const MIN_HEALTH_FACTOR: u8 = 80;
