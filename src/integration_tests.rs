#[cfg(test)]
mod tests {
    use crate::helpers::StakingContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::testing::{ mock_env};
    use cosmwasm_std::{Addr, Coin, Empty, Uint128, Decimal, Validator, coin};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor, StakingInfo};

    pub fn contract_staking() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER1: &str = "USER1";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "ujunox";

    const VALIDATOR1: &str = "validator1";
    const VALIDATOR2: &str = "validator2";
    const VALIDATOR3: &str = "validator3";


    fn mock_app() -> App {
        AppBuilder::new().build(|router, api, storage| {
            let env = mock_env();
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER1),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(2000),
                    }],
                )
                .unwrap();
        // Setup staking module for the correct mock data.                
        router
                .staking
                .setup(
                    storage,
                    StakingInfo {
                        bonded_denom: NATIVE_DENOM.to_string(),
                        unbonding_time: 1,
                        apr: Decimal::percent(10),
                    },
                )
                .unwrap();
        // Add mock validators
        router
            .staking
            .add_validator(
                api,
                storage,
                &env.block,
                Validator {
                    address: VALIDATOR1.to_string(),
                    commission: Decimal::zero(),
                    max_commission: Decimal::one(),
                    max_change_rate: Decimal::one(),
                },
            )
            .unwrap();
        router
            .staking
            .add_validator(
                api,
                storage,
                &env.block,
                Validator {
                    address: VALIDATOR2.to_string(),
                    commission: Decimal::zero(),
                    max_commission: Decimal::one(),
                    max_change_rate: Decimal::one(),
                },
            )
            .unwrap();
        router
            .staking
            .add_validator(
                api,
                storage,
                &env.block,
                Validator {
                    address: VALIDATOR3.to_string(),
                    commission: Decimal::zero(),
                    max_commission: Decimal::one(),
                    max_change_rate: Decimal::one(),
                },
            )
            .unwrap();
        })
    }

    fn staking_instantiate() -> (App, StakingContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_staking());

        let msg = InstantiateMsg { };
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "sim-staking",
                None,
            )
            .unwrap();

        let cw_template_contract = StakingContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn bond() {
            let (mut app, staking_contract) = staking_instantiate();

            let balance = app.wrap().query_balance(staking_contract.addr(), NATIVE_DENOM).unwrap();
            println!("######### INITIAL - BALANCE: {:?}", balance);

            let msg = ExecuteMsg::Transfer { };
            app.execute_contract(Addr::unchecked(USER1), staking_contract.addr(), &msg, &[coin(600, NATIVE_DENOM.to_string())]).unwrap();

            let balance = app.wrap().query_balance(staking_contract.addr(), NATIVE_DENOM).unwrap();
            println!("######### USER SENDS 600 TOKENS - BALANCE: {:?}", balance);

            // Delegate the hard coded 100 tokens
            let msg = ExecuteMsg::Bond { val_addr: VALIDATOR1.to_string() };
            app.execute_contract(Addr::unchecked(USER1), staking_contract.addr(), &msg, &[]).unwrap();

            let balance = app.wrap().query_balance(staking_contract.addr(), NATIVE_DENOM).unwrap();
            println!("######### CONTRACT STAKES 100 TOKENS - BALANCE: {:?}", balance);

            let delegation = app.wrap().query_delegation(staking_contract.addr(), VALIDATOR1.to_string()).unwrap().unwrap();
            println!("######### VALIDATOR1 BONDED FROM CONTRACT - BALANCE: {:?}", delegation.amount);

            // Unbond the hard coded 100 tokens
            let msg = ExecuteMsg::Unbond { val_addr: VALIDATOR1.to_string() };
            app.execute_contract(Addr::unchecked(USER1), staking_contract.addr(), &msg, &[]).unwrap();

            app.update_block(|block| block.time = block.time.plus_seconds(60 * 60 * 24 * 31 * 12 ));


            let delegation = app.wrap().query_delegation(staking_contract.addr(), VALIDATOR1.to_string()).unwrap();
            println!("######### VALIDATOR1 BONDED FROM CONTRACT AFTER UNBONDING 100 TOKENS - BALANCE: {:?}", delegation);

            let balance = app.wrap().query_balance(staking_contract.addr(), NATIVE_DENOM).unwrap();
            println!("######### CONTRACT BALANCE AFTER UNBONDING 100 TOKENS A YEAR LATER  - BALANCE: {:?}", balance);

        }
    }
}
