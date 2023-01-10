pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg};
pub use crate::state::{ADR36SignDoc, Fee, MsgSignData, MsgSignDataValue, PermitSignature};
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
pub use cw721_base::{
    entry::{execute as _execute, query as _query},
    state::TokenInfo,
    ContractError as Cw721ContractError, Cw721Contract, ExecuteMsg as Cw721BaseExecuteMsg,
    Extension, InstantiateMsg as Cw721BaseInstantiateMsg, MintMsg, MinterResponse,
    QueryMsg as Cw721QueryMsg,
};

use bech32::{ToBase32, Variant::Bech32};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use std::{env, str};

pub mod error;
pub mod msg;
pub mod state;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw721-whitelist";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const AGREEMENT_STRING: &str =
    "Agreement(string chain_id,address active,address passive,string tokenURI)";

pub type Cw4973Contract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;
#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        // set contract's information
        let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            minter: msg.minter,
        };

        Cw4973Contract::default().instantiate(
            deps.branch(),
            env,
            info,
            cw721_base_instantiate_msg,
        )?;

        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default()
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::AddWhitelist { address } => {
                Cw4973Contract::default().add_whitelist(deps, env, info, address)
            }
            ExecuteMsg::RemoveWhitelist { address } => {
                Cw4973Contract::default().remove_whitelist(deps, env, info, address)
            }
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw4973Contract::default().query(deps, env, msg)
    }
}

// function allow minter to add addresses to the whitelist
pub fn add_whitelist(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: Vec<String>,
) -> Result<Response, ContractError> {
    // check if the sender is the minter
    let minter = Cw4973Contract::default().minter(deps.storage)?;
    if info.sender != minter.minter {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::new()
        .add_attribute("action", "add_whitelist")
        .add_attribute("minter", info.sender)
        .add_attribute("address", address.join(",")))
}

// rewrite mint function of cw721 base to ignore minter checking
fn _mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: MintMsg<Extension>,
) -> Result<Response, ContractError> {
    // create the token
    let token = TokenInfo {
        owner: deps.api.addr_validate(&msg.owner)?,
        approvals: vec![],
        token_uri: msg.token_uri,
        extension: msg.extension,
    };

    // update tokens list of contract
    Cw4973Contract::default()
        .tokens
        .update(deps.storage, &msg.token_id, |old| match old {
            Some(_) => Err(ContractError::Claimed),
            None => Ok(token),
        })?;
    Cw4973Contract::default().increment_tokens(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("minter", info.sender)
        .add_attribute("owner", msg.owner)
        .add_attribute("token_id", msg.token_id))
}
