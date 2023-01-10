use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use cw721_base::QueryMsg;
use cw721_whitelist::msg::{ExecuteMsg, InstantiateMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg<Empty>
    }
}
