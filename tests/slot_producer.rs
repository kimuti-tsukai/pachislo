use std::time::Instant;

use pachislo::slot::SlotProducer;
use rand::rngs::ThreadRng;

#[test]
fn win() {
    let mut slot_producer: SlotProducer<u8, ThreadRng> = SlotProducer::new(3, (1..=9).collect());

    for _ in 0..1000 {
        let mut slot = slot_producer.produce_win();

        assert_eq!(slot.len(), 3);

        slot.sort_unstable();
        slot.dedup();

        assert_eq!(slot.len(), 1);
    }
}

#[test]
fn lose() {
    let start = Instant::now();

    // 全ての可能な入力パターンをテスト
    let test_cases = vec![
        (2, vec![1, 2]),          // 最小ケース
        (3, vec![1, 2]),          // length > choices
        (3, vec![1, 2, 3, 4, 5]), // 通常ケース
    ];

    for (length, choices) in test_cases {
        let mut producer: SlotProducer<u8, ThreadRng> = SlotProducer::new(length, choices.clone());

        // 決定論的に検証可能な条件をテスト
        for _ in 0..1_000_000 {
            // 十分な回数
            let result = producer.produce_lose();

            // 基本的な不変条件をチェック
            assert_eq!(result.len(), length);
            assert!(result.iter().all(|x| choices.contains(x)));

            // 「負け」条件をチェック（全て同じでないこと）
            let first = result[0];
            assert!(
                !result.iter().all(|&x| x == first),
                "All elements are same: {result:?}",
            );
        }
    }

    let end = start.elapsed();
    println!("Elapsed time: {end:?}");
}

#[test]
fn big_lose() {
    let mut producer: SlotProducer<u32, ThreadRng> =
        SlotProducer::new(10000, (0..1000000).collect());

    let start = Instant::now();

    for _ in 0..100 {
        let _result = producer.produce_lose();
    }

    let end = start.elapsed();
    println!("Elapsed time: {end:?}");
}
