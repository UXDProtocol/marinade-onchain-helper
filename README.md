Forked marinade CPI helper and mixed with the `State` part of the https://github.com/marinade-finance/liquid-staking-program with anchor upgraded. It's a way to interact with mariande for now until they release a anchor-independent sdk / or anchor upgrade on their side

# Marinade Finance on-chain CPI helper

This Rust lib will simplify integrating CPI calls by giving out the structure account and invoke signed function

## Usage

add to your .toml file

```
marinade-onchain-helper = { git = "https://github.com/marinade-finance/marinade-onchain-helper" }
```

Add to the begining of your file use (remove unused functions):

```
use marinade_onchain_helper::{
    cpi_context_accounts::{
        MarinadeDeposit, 
        MarinadeDepositStakeAccount, 
        MarinadeLiquidUnstake
    },
    cpi_util::{
        invoke_signed,
    }
};
```
