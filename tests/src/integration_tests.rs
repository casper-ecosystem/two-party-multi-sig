#[cfg(test)]
mod tests {
    
    use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_RUN_GENESIS_REQUEST, DEFAULT_ACCOUNT_ADDR};
    use casper_types::{runtime_args, RuntimeArgs};
    use casper_types::account::{AccountHash};

    const ASSOCIATED_ACCOUNT_HASH: AccountHash = AccountHash::new([1u8; 32]); // hash of the associated account
    const ASSOCIATED_ACCOUNT: &str = "deployment-account";  // the associated account argument
    const CONTRACT_WASM: &str = "contract.wasm";            // file to pass to the instance of the EE


    #[test]
    fn should_add_associated_key() {
        // Initialize an instance of the execution engine and assign it to the builder variable
        let mut builder = InMemoryWasmTestBuilder::default();

        // Execute the genesis process
        builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

        // Retrieve runtime arguments. These should be same as defined in the contract
        // This allows use to check and assert behavior of the session code
        let runtime_args = runtime_args! {
            ASSOCIATED_ACCOUNT => ASSOCIATED_ACCOUNT_HASH
        };

        // Create the execution request that will eventually be executed by the EE
        // Load the session wasm and pass in the runtime arguments
        // Sets up the session code to be executed in the default account using auth keys and default account address
        let execute_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            CONTRACT_WASM,
            runtime_args,
        )
        .build();

        // Invoke the EE to execute the session code that we are testing
        builder
            .exec(execute_request)
            .expect_success()
            .commit();

        // Verify the results of the execution match our expectations from the contract using the test results

        let _account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have a primary account");

        let _associated_account = builder
        .get_account(ASSOCIATED_ACCOUNT_HASH)
        .expect("should have an associated account");
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
