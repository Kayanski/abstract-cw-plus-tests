use cw20::Cw20Coin;
use cw20_base::msg::QueryMsgFns;
use cw20::msg::Cw20ExecuteMsgFns;
use boot_core::*;

use cosmwasm_std::Addr;
use cw20_base::contract::Cw20Base;

// shows how to deploy CwPlus to a mock environment
fn main() -> anyhow::Result<()> {
    // set a sender address
    let sender = Addr::unchecked("test_sender");
    // get the mock environment
    let (_, mock) = instantiate_default_mock_env(&sender)?;
    // identical code to the chain example
    cw20_example(mock)
}

fn cw20_example<Chain: CwEnv>(chain: Chain) -> anyhow::Result<()> {
    let sender = chain.sender();
    // Upload the cw-plus contracts
    let mut cw20_base_contracts = Cw20Base::new("cw20_base_mock", chain.clone());
    cw20_base_contracts.upload()?;
    // instantiate an instance of it
    let cw20_init_msg = cw20_base::msg::InstantiateMsg {
        decimals: 6,
        name: "Test Token".to_string(),
        initial_balances: vec![Cw20Coin {
            address: sender.to_string(),
            amount: 1000000u128.into(),
        }],
        marketing: None,
        mint: None,
        symbol: "TEST".to_string(),
    };
    cw20_base_contracts.instantiate(&cw20_init_msg, None, None)?;

    // send some tokens
    let cw20_send_msg = cw20_base::msg::ExecuteMsg::Transfer {
        recipient: "recipient_addr".to_string(),
        amount: 100u128.into(),
    };
    cw20_base_contracts.execute(&cw20_send_msg, None)?;

    // query the balance of the recipient
    let query_msg = cw20_base::msg::QueryMsg::Balance {
        address: "recipient_addr".to_string(),
    };
    let _balance: cw20::BalanceResponse = cw20_base_contracts.query(&query_msg)?;

    // query balance after init
    // notice that this query is generated by the macro and not defined in the object itself!
    let balance = cw20_base_contracts.balance(sender.to_string())?;
    assert_eq!(balance.balance.u128(), 999900u128.into());

    // Send with the macro-generated function
    let transfer_resp = cw20_base_contracts.transfer(100u128.into(), "recipient_addr".to_string())?;
    assert_eq!(
        // index the response
        transfer_resp.event_attr_value("wasm", "amount")?,
        100.to_string()
    );

    Ok(())
}
