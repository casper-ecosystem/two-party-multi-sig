#[cfg(test)]
mod tests {
    
    use std::path::PathBuf;
    use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_RUN_GENESIS_REQUEST, DEFAULT_ACCOUNT_ADDR};
    use casper_types::{runtime_args};

    const ASSOCIATED_ACCOUNT: &str = "deployment-account";  // the associated account
    const CONTRACT_WASM: &str = "contract.wasm";            // file to pass to the instance of the EE

    #[test]
    fn should_add_associated_key() {
        // Initialize an instance of the execution engine and assign it to the builder variable
        let mut builder = InMemoryWasmTestBuilder::default();

        // Execute the genesis process
        builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

        // Retrieve the contract wasm from the specified location and assign to the session code variable
        let session_code = PathBuf::from(CONTRACT_WASM);

        // Retrieve runtime arguments. These should be same as defined in the contract
        // This allows use to check and assert behavior of the session code
        let runtime_args = runtime_args! {
            ASSOCIATED_ACCOUNT => 1
        };

        // Create the execution request that will eventually be executed by the EE
        // Load the session wasm and pass in the runtime arguments
        // Sets up the session code to be executed in the default account using auth keys and default account address
        let execute_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_DEFINE_WASM,
            runtime_args,
        )
        .build();

        // Invoke the EE to execute the session code that we are testing
        builder
            .exec(execute_request)
            .expect_success()
            .commit();

        // Verify the results of the execution match our expectations from the contract using the test results

        let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
