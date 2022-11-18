#[cfg(test)]
mod tests {
    use core::panic;
    use base58check::ToBase58Check;
    use osc_api::models::{BlockStatus, SignalStats, SignalStatus};
    use osc_api::routes::processor::SignalProcessor;
    use osc_api::{
        ledger::{HasConnection, Ledger},
        models::DBResponse
    };

    pub fn mina_encode(memo: &str) -> String {
        let bytes = memo.as_bytes();
        let mut encoded = Vec::new();
        encoded.push(1);
        encoded.push(memo.len() as u8);
        for byte in bytes.iter() {
            encoded.push(*byte);
        }

        encoded.as_slice().to_base58check(0)
    }

    pub fn with_ledger_mock<T, C>(mock: &str, test: T) -> ()
    where
        Ledger<C>: HasConnection,
        T: FnOnce(Ledger<C>) -> () + panic::UnwindSafe,
    {
        let filename = format!("./tests/__mocks__/{}.json", mock);
        let ledger = HasConnection::init(&filename).unwrap();
        test(ledger)
    }

    pub fn test_signal_stats(
        mock: &str,
        signal: DBResponse,
        key: &str,
        latest_block: i64,
        signal_stats: SignalStats,
    ) {
        with_ledger_mock(mock, |ledger| {
            let mut conn = ledger.db;
            let signals = vec![signal];
            let response_entity = SignalProcessor::new(Box::new(&mut conn), &key, latest_block, signals).run();

            assert_eq!(signal_stats, response_entity.stats.unwrap());
        });
    }

    pub fn test_signal_status(
        mock: &str,
        signal: DBResponse,
        key: &str,
        latest_block: i64,
        signal_status: SignalStatus,
    ) {
        with_ledger_mock(mock, |ledger| {
            let mut conn = ledger.db;
            let signals = vec![signal];
            let response_entity = SignalProcessor::new(Box::new(&mut conn), &key, latest_block, signals).run();

            match signal_status {
                SignalStatus::Settled => assert!(response_entity.settled.len() > 0),
                SignalStatus::Unsettled => assert!(response_entity.unsettled.len() > 0),
                SignalStatus::Invalid => assert!(response_entity.invalid.len() > 0),
            }
        });
    }

    #[test]
    pub fn undelegated_account_unsettled_signal_adds_balance_to_yes() {
        let mock = "undelegated_account";
        let key = "magenta";
        let signal = DBResponse {
            account: "A".to_string(),
            memo: mina_encode("magenta"),
            height: 0,
            status: osc_api::models::BlockStatus::Canonical,
            timestamp: 0,
        };
        test_signal_stats(
            mock,
            signal.clone(),
            key,
            0,
            SignalStats { yes: 1.0, no: 0.0 },
        );
        test_signal_status(mock, signal, key, 0, SignalStatus::Unsettled);
    }

    #[test]
    pub fn delegated_account_unsettled_signal_adds_balance_to_yes() {
        let mock = "delegated_account";
        let key = "magenta";
        let signal = DBResponse {
            account: "A".to_string(),
            memo: mina_encode("magenta"),
            height: 0,
            status: osc_api::models::BlockStatus::Canonical,
            timestamp: 0,
        };
        test_signal_stats(
            "delegated_account",
            signal.clone(),
            "magenta",
            0,
            SignalStats { yes: 1.0, no: 0.0 },
        );
        test_signal_status(mock, signal, key, 0, SignalStatus::Unsettled);
    }

    #[test]
    pub fn undelegated_account_unsettled_signal_adds_balance_to_no() {
        let mock = "undelegated_account";
        let key = "magenta";
        let signal = DBResponse {
            account: "A".to_string(),
            memo: mina_encode("no magenta"),
            height: 0,
            status: osc_api::models::BlockStatus::Canonical,
            timestamp: 0,
        };
        test_signal_stats(
            mock,
            signal.clone(),
            key,
            0,
            SignalStats { yes: 0.0, no: 1.0 },
        );
        test_signal_status(mock, signal, key, 0, SignalStatus::Unsettled);
    }

    #[test]
    pub fn delegated_account_unsettled_signal_adds_balance_to_no() {
        let mock = "delegated_account";
        let key = "magenta";
        let signal = DBResponse {
            account: "A".to_string(),
            memo: mina_encode("no magenta"),
            height: 0,
            status: BlockStatus::Canonical,
            timestamp: 0,
        };
        test_signal_stats(
            mock,
            signal.clone(),
            key,
            0,
            SignalStats { yes: 0.0, no: 1.0 },
        );
        test_signal_status(mock, signal, key, 0, SignalStatus::Unsettled);
    }

    #[test]
    pub fn undelegated_account_settled_signal_adds_balance_to_yes() {
        let mock = "undelegated_account";
        let key = "magenta";
        let signal = DBResponse {
            account: "A".to_string(),
            memo: mina_encode("magenta"),
            height: 0,
            status: BlockStatus::Canonical,
            timestamp: 0,
        };
        test_signal_stats(
            mock,
            signal.clone(),
            "magenta",
            20,
            SignalStats { yes: 1.0, no: 0.0 },
        );
        test_signal_status(mock, signal, key, 20, SignalStatus::Settled);
    }

    #[test]
    pub fn delegated_account_settled_signal_adds_balance_to_yes() {
        let mock = "delegated_account";
        let key = "magenta";
        let signal = DBResponse {
            account: "A".to_string(),
            memo: mina_encode("magenta"),
            height: 0,
            status: BlockStatus::Canonical,
            timestamp: 0,
        };
        test_signal_stats(
            mock,
            signal.clone(),
            "magenta",
            20,
            SignalStats { yes: 1.0, no: 0.0 },
        );
        test_signal_status(mock, signal, key, 20, SignalStatus::Settled);
    }

    #[test]
    pub fn undelegated_account_settled_signal_adds_balance_to_no() {
        let mock = "undelegated_account";
        let key = "magenta";
        let signal = DBResponse {
            account: "A".to_string(),
            memo: mina_encode("no magenta"),
            height: 0,
            status: BlockStatus::Canonical,
            timestamp: 0,
        };
        test_signal_stats(
            mock,
            signal.clone(),
            "magenta",
            20,
            SignalStats { yes: 0.0, no: 1.0 },
        );
        test_signal_status(mock, signal, key, 20, SignalStatus::Settled);
    }

    #[test]
    pub fn delegated_account_settled_signal_adds_balance_to_no() {
        let mock = "delegated_account";
        let key = "magenta";
        let signal = DBResponse {
            account: "A".to_string(),
            memo: mina_encode("no magenta"),
            height: 0,
            status: BlockStatus::Canonical,
            timestamp: 0,
        };
        test_signal_stats(
            mock,
            signal.clone(),
            "magenta",
            20,
            SignalStats { yes: 0.0, no: 1.0 },
        );
        test_signal_status(mock, signal, key, 20, SignalStatus::Settled);
    }

    #[test]
    pub fn delegated_account_invalid_signal() {
        let mock = "delegated_account";
        let key = "magenta";
        let signal = DBResponse {
            account: "A".to_string(),
            memo: mina_encode("invalid magenta signal"),
            height: 0,
            status: BlockStatus::Canonical,
            timestamp: 0,
        };

        test_signal_status(mock, signal, key, 0, SignalStatus::Invalid);
    }

    #[test]
    pub fn undelegated_account_invalid_signal() {
        let mock = "undelegated_account";
        let key = "magenta";
        let signal = DBResponse {
            account: "A".to_string(),
            memo: mina_encode("invalid magenta signal"),
            height: 0,
            status: BlockStatus::Canonical,
            timestamp: 0,
        };

        test_signal_status(mock, signal, key, 0, SignalStatus::Invalid);
    }

    #[test]
    pub fn repeat_signals_use_hash_map() {
        // this test mainly is used to verify that
        // signalling still works when using the
        // internal hashmap to store accounts
        // an account is added to this hashmap
        // after the first time a signal from it
        // is processed, thus only the second
        // signal from any said account will
        // query the internal hashmap caching system
        // instead of asking the ledger
        with_ledger_mock("delegated_account", |ledger| {
            let mut conn = ledger.db;
            let key = "magenta";
            let signal = DBResponse {
                account: "A".to_string(),
                memo: mina_encode("magenta"),
                height: 0,
                status: BlockStatus::Canonical,
                timestamp: 0,
            };
            let signals = vec![signal.clone(), signal];
            let response_entity =
            SignalProcessor::new(Box::new(&mut conn), &key, 0, signals).run();

            assert_eq!(
                SignalStats { yes: 2.0, no: 0.0 },
                response_entity.stats.unwrap()
            );
        });
    }

    #[test]
    pub fn repeat_settled_signals_bypass_signal_status_check() {
        // this test ensures that
        // if a signal is processed from an
        // account that has already had a signal
        // processed, the signal is not again
        // reclassified as Settled or Unsettled
        with_ledger_mock("delegated_account", |ledger| {
            let mut conn = ledger.db;
            let key = "magenta";
            let signal = DBResponse {
                account: "A".to_string(),
                memo: mina_encode("magenta"),
                height: 0,
                status: BlockStatus::Canonical,
                timestamp: 0,
            };
            let signals = vec![signal.clone(), signal];
            let response_entity =
            SignalProcessor::new(&mut conn, &key, 20, signals).run();

            assert!(response_entity.settled.len() > 0);
            assert_eq!(
                SignalStats { yes: 2.0, no: 0.0 },
                response_entity.stats.unwrap()
            );
        });
    }

    // #[test]
    // pub fn ledger_init_creates_connection() {
    //     let _devnet: Ledger<tokio_rusqlite::Connection> =
    //         pollster::block_on(HasConnectionAsync::init_async("./devnet_ledger.json")).unwrap();
    //     let _mainnet: Ledger<tokio_rusqlite::Connection> =
    //         pollster::block_on(HasConnectionAsync::init_async("./mainnet_ledger.json")).unwrap();
    // }
}
