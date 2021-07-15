// Copyright (c) 2021 MASSA LABS <info@massa.net>

use std::collections::HashMap;

use crate::tests::tools::{self, generate_ledger_file};
use models::Slot;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_pruning_of_discarded_blocks() {
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<crypto::signature::PrivateKey> = (0..1)
        .map(|_| crypto::generate_random_private_key())
        .collect();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.future_block_processing_max_periods = 50;
    cfg.max_future_processing_blocks = 10;

    tools::consensus_without_pool_test(
        cfg.clone(),
        None,
        async move |mut protocol_controller, consensus_command_sender, consensus_event_receiver| {
            let parents = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status")
                .best_parents;

            // Send more bad blocks than the max number of cached discarded.
            for i in 0..(cfg.max_discarded_blocks + 5) as u64 {
                // Too far into the future.
                let _ = tools::create_and_test_block(
                    &mut protocol_controller,
                    &cfg,
                    Slot::new(100000000 + i, 0),
                    parents.clone(),
                    false,
                    false,
                    staking_keys[0].clone(),
                )
                .await;
            }

            let status = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status");
            assert!(status.discarded_blocks.map.len() <= cfg.max_discarded_blocks);

            (
                protocol_controller,
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_pruning_of_awaiting_slot_blocks() {
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<crypto::signature::PrivateKey> = (0..1)
        .map(|_| crypto::generate_random_private_key())
        .collect();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.future_block_processing_max_periods = 50;
    cfg.max_future_processing_blocks = 10;

    tools::consensus_without_pool_test(
        cfg.clone(),
        None,
        async move |mut protocol_controller, consensus_command_sender, consensus_event_receiver| {
            let parents = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status")
                .best_parents;

            // Send more blocks in the future than the max number of future processing blocks.
            for i in 0..(cfg.max_future_processing_blocks + 5) as u64 {
                // Too far into the future.
                let _ = tools::create_and_test_block(
                    &mut protocol_controller,
                    &cfg,
                    Slot::new(10 + i, 0),
                    parents.clone(),
                    false,
                    false,
                    staking_keys[0].clone(),
                )
                .await;
            }

            let status = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status");
            assert!(status.discarded_blocks.map.len() <= cfg.max_future_processing_blocks);
            (
                protocol_controller,
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_pruning_of_awaiting_dependencies_blocks_with_discarded_dependency() {
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<crypto::signature::PrivateKey> = (0..1)
        .map(|_| crypto::generate_random_private_key())
        .collect();
    let staking_file = tools::generate_staking_keys_file(&staking_keys);

    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 200.into();
    cfg.future_block_processing_max_periods = 50;
    cfg.max_future_processing_blocks = 10;

    tools::consensus_without_pool_test(
        cfg.clone(),
        None,
        async move |mut protocol_controller, consensus_command_sender, consensus_event_receiver| {
            let parents = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status")
                .best_parents;

            // Too far into the future.
            let (bad_parent, bad_block, _) = tools::create_block(
                &cfg,
                Slot::new(10000, 0),
                parents.clone(),
                staking_keys[0].clone(),
            );

            for i in 1..4 {
                // Sent several headers with the bad parent as dependency.
                let _ = tools::create_and_test_block(
                    &mut protocol_controller,
                    &cfg,
                    Slot::new(i, 0),
                    vec![bad_parent.clone(), parents.clone()[0]],
                    false,
                    false,
                    staking_keys[0].clone(),
                )
                .await;
            }

            // Now, send the bad parent.
            protocol_controller.receive_header(bad_block.header).await;
            tools::validate_notpropagate_block_in_list(
                &mut protocol_controller,
                &vec![bad_parent],
                10,
            )
            .await;

            // Eventually, all blocks will be discarded due to their bad parent.
            // Note the parent too much in the future will not be discarded, but ignored.
            loop {
                let status = consensus_command_sender
                    .get_block_graph_status()
                    .await
                    .expect("could not get block graph status");
                if status.discarded_blocks.map.len() == 3 {
                    break;
                }
            }
            (
                protocol_controller,
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}
