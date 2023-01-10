use crate::state::PermitSignature;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // allow the minter add addresses to the whitelist
    AddWhitelist { address: Vec<String> },

    // allow the minter remove addresses from the whitelist
    RemoveWhitelist { address: Vec<String> },
}
