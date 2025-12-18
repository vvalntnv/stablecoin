pub const CONFIG_SEED: &[u8] = b"config";
pub const COLLAT_SEED: &[u8] = b"collateral";
pub const MINT_SEED: &[u8] = b"mint";
pub const RESERVE_ACCOUNT_SEED: &[u8] = b"reserve-account";

// all in percentage
pub const LIQUIDATION_THRESHOLD: u8 = 50;
pub const LIQUIDATION_BONUS: u8 = 10;
pub const LIQUIDATION_FEE: u8 = 2; 

// If amount minted passes 80% of the collateral debited (collateral changes price),
// then the collateral + (collateral * liq_bonus) are sold to the market
pub const MIN_HEALTH_FACTOR: u8 = 80;

pub const SOLANA_USD_PRICE_FEED: &str =
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

pub const PRICE_MAX_AGE: u64 = 30;
