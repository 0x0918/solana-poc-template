use solana_poc_framework::{solana_sdk::signature::Keypair, LocalEnvironment};

struct Challenge {
    // hacker: Keypair,
    // wallet_program: Pubkey,
    // wallet_address: Pubkey,
    // wallet_owner: Pubkey,
    // wallet_authority: Pubkey,
    // mint: Pubkey,
    // tip_program: Pubkey,
    // initizalizer: Pubkey,
    // poor_boi: Pubkey,
    // rich_boi: Pubkey,
    // tip_pool: Pubkey,
    // vault_address: Pubkey,
}
/** hacking */
fn hack(env: &mut LocalEnvironment, challenge: &Challenge) {}

/*
SETUP CODE BELOW
*/
fn main() {
    // let (mut env, challenge, internal) = setup();
    // hack(&mut env, &challenge);
    // verify(&mut env, challenge, internal);
}

struct Internal {
    wallet_owner: Keypair,
    wallet_amount: u64,
}

fn verify(env: &mut LocalEnvironment, challenge: Challenge, internal: Internal) {}

// fn setup() -> (LocalEnvironment, Challenge, Internal) {}
