use factors_methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::{
    default_executor_from_elf,
    serde::{from_slice, to_vec},
    ExecutorEnv,
};

fn main() {
    let a: u64 = 17;
    let b: u64 = 23; 
    // First, we construct an executor environment
    //let env = ExecutorEnv::builder().build().unwrap();
    let env = ExecutorEnv::builder()
    // Send a & b to the guest
    .add_input(&to_vec(&a).unwrap())
    .add_input(&to_vec(&b).unwrap())
    .build()
    .unwrap();


    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example:
    // let env = ExecutorEnv::builder().add_input(&vec).build().unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = default_executor_from_elf(env, MULTIPLY_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(MULTIPLY_ID).unwrap();
}
