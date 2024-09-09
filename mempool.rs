
use solana_sdk::transaction::Transaction;
use solana_sdk::commitment_config::CommitmentConfig;
use log::info;
use std::error::Error;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{EncodedConfirmedBlock, EncodedTransactionWithStatusMeta};

pub async fn fetch_pending_transactions(client: &RpcClient) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let confirmed_block: EncodedConfirmedBlock = client.get_block(1).await?;
    let mut transactions = Vec::new();

    for tx_with_meta in confirmed_block.transactions {
        if let Some(transaction) = tx_with_meta.transaction {
            let parsed_tx = parse_encoded_transaction(transaction)?;
            transactions.push(parsed_tx);
        }
    }

    Ok(transactions)
}

fn parse_encoded_transaction(encoded_tx: EncodedTransactionWithStatusMeta) -> Result<Transaction, Box<dyn Error>> {
    // Implement transaction parsing logic
    let decoded_tx: Vec<u8> = base64::decode(encoded_tx.transaction).expect("Failed to decode transaction");
    // Assuming the transaction is deserialized here
    let tx: Transaction = bincode::deserialize(&decoded_tx).expect("Failed to deserialize transaction");
    Ok(tx)
}
