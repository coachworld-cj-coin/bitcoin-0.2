use time::OffsetDateTime;

use crate::{
    block::{Block, BlockHeader},
    transaction::{Transaction, TxOutput},
    reward::block_reward,
    consensus::difficulty::calculate_next_target,
    merkle::merkle_root,
    pow::mine,
    validation::validate_transaction,
    utxo::UTXOSet,
    policy::{MAX_BLOCK_TXS, MAX_BLOCK_TX_BYTES},
};

const MIN_FEE_PER_BYTE: i64 = 1; // POLICY ONLY

pub fn mine_block(
    prev_block: &Block,
    utxos: &UTXOSet,
    mempool_txs: Vec<Transaction>,
    miner_pubkey_hash: Vec<u8>,
    chain: &[Block],
) -> Block {
    let height = prev_block.header.height + 1;

    let coinbase = Transaction {
        inputs: vec![],
        outputs: vec![TxOutput {
            value: block_reward(height),
            pubkey_hash: miner_pubkey_hash,
        }],
    };

    let mut selected = vec![coinbase];
    let mut total_bytes = selected[0].serialized_size();

    for tx in mempool_txs {
        if selected.len() >= MAX_BLOCK_TXS {
            break;
        }

        let size = tx.serialized_size();
        if total_bytes + size > MAX_BLOCK_TX_BYTES {
            break;
        }

        if !validate_transaction(&tx, utxos, height) {
            continue;
        }

        let mut input = 0i64;
        let mut output = 0i64;

        for i in &tx.inputs {
            let key = format!(
                "{}:{}",
                hex::encode(&i.txid),
                i.index
            );
            if let Some(u) = utxos.get(&key) {
                input += u.value as i64;
            }
        }

        for o in &tx.outputs {
            output += o.value as i64;
        }

        let fee = input - output;
        if fee <= 0 {
            continue;
        }

        let fee_rate = fee / size as i64;
        if fee_rate < MIN_FEE_PER_BYTE {
            continue;
        }

        total_bytes += size;
        selected.push(tx);
    }

    let target = calculate_next_target(chain);

    let mut block = Block {
        header: BlockHeader {
            height,
            timestamp: OffsetDateTime::now_utc().unix_timestamp(),
            prev_hash: prev_block.hash.clone(),
            nonce: 0,
            target,
            merkle_root: merkle_root(&selected),
        },
        transactions: selected,
        hash: vec![],
    };

    mine(&mut block);
    block
}
