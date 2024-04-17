use anchor_lang::prelude::Pubkey;
use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program_test::ProgramTest;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::system_instruction;
use solana_sdk::{instruction::Instruction, signature::Signer, transaction::Transaction};
use with_account::{ID, NOOP_PROGRAM_ID};
// Hardcoded keypairs for deterministic pubkeys for testing
pub const MERKLE_TREE_TEST_KEYPAIR: [u8; 64] = [
    146, 193, 80, 51, 114, 21, 221, 27, 228, 203, 43, 26, 211, 158, 183, 129, 254, 206, 249, 89,
    121, 99, 123, 196, 106, 29, 91, 144, 50, 161, 42, 139, 68, 77, 125, 32, 76, 128, 61, 180, 1,
    207, 69, 44, 121, 118, 153, 17, 179, 183, 115, 34, 163, 127, 102, 214, 1, 87, 175, 177, 95, 49,
    65, 69,
];

#[tokio::test]
async fn test_append_leaves() {
    let mut program_test = ProgramTest::default();
    program_test.add_program("with_account", ID, None);
    program_test.add_program("spl_noop", NOOP_PROGRAM_ID, None);
    program_test.set_compute_max_units(1_400_000u64);

    let mut context = program_test.start_with_context().await;
    let merkle_tree_keypair = Keypair::from_bytes(&MERKLE_TREE_TEST_KEYPAIR).unwrap();

    let account_create_ix = crate::create_account_instruction(
        &context.payer.pubkey(),
        10_000 + 8,
        context
            .banks_client
            .get_rent()
            .await
            .unwrap()
            .minimum_balance(32_000 + 8),
        &ID,
        Some(&merkle_tree_keypair),
    );

    let instruction_data = with_account::instruction::AppendLeaves {};
    let accounts = with_account::accounts::AppendLeaves {
        user: context.payer.pubkey(),
        log_wrapper: NOOP_PROGRAM_ID,
        heap_storage: merkle_tree_keypair.pubkey(),
    };
    let instruction = Instruction {
        program_id: ID,
        accounts: accounts.to_account_metas(Some(true)),
        data: instruction_data.data(),
    };
    let transaction = Transaction::new_signed_with_payer(
        &[account_create_ix, instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &merkle_tree_keypair],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}
pub fn create_account_instruction(
    payer: &Pubkey,
    size: usize,
    rent: u64,
    id: &Pubkey,
    keypair: Option<&Keypair>,
) -> Instruction {
    let keypair = match keypair {
        Some(keypair) => keypair.insecure_clone(),
        None => Keypair::new(),
    };
    system_instruction::create_account(payer, &keypair.pubkey(), rent, size as u64, id)
}
