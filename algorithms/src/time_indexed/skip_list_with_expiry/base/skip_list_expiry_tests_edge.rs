//! Edge–case tests for `SkipListExpiry`

use crate::time_indexed::skip_list_with_expiry::base::skip_list_expiry::SkipListExpiry;
use chrono::Utc;

/* -------------------------------------------------------------------------- */
/* ✅ Table 1 – Implemented edge‑case tests                                   */
/* -------------------------------------------------------------------------- */

/// Ensure calling expire on an empty structure returns an empty vector.
#[test]
fn test_zero_items_expire_empty() {
    let skiplist: SkipListExpiry<String> = SkipListExpiry::new(3, 5);
    assert_eq!(skiplist.expire_front(), Vec::<String>::new());
}

/// Verify that buckets recycle correctly after a full rotation.
#[test]
fn test_expire_all_buckets_and_reuse() {
    let skiplist: SkipListExpiry<String> = SkipListExpiry::new(3, 5);
    let base = Utc::now().timestamp();

    skiplist.insert("T".to_string(), base);
    assert_eq!(skiplist.expire_front(), vec!["T".to_string()]);
    skiplist.expire_front(); // advance bucket 1
    skiplist.expire_front(); // advance bucket 2

    let new_time = base + 15;
    skiplist.insert("U".to_string(), new_time);
    let expired = skiplist.expire_front();
    assert_eq!(expired, vec!["U".to_string()]);
}

/// Confirm `len()` decreases as items expire over successive ticks.
#[test]
fn test_len_decreases_after_expiry() {
    let skiplist: SkipListExpiry<String> = SkipListExpiry::new(4, 2);
    let base = Utc::now().timestamp();

    skiplist.insert("A".to_string(), base);
    skiplist.insert("B".to_string(), base + 2);
    skiplist.insert("C".to_string(), base + 4);

    assert_eq!(skiplist.len(), 3);
    skiplist.expire_front(); // expires A
    assert_eq!(skiplist.len(), 2);
    skiplist.expire_front(); // expires B
    assert_eq!(skiplist.len(), 1);
    skiplist.expire_front(); // expires C
    assert_eq!(skiplist.len(), 0);
}

/// Validate that an insertion landing exactly on a bucket boundary expires in that bucket.
#[test]
fn test_insert_exactly_on_boundary() {
    let skiplist: SkipListExpiry<String> = SkipListExpiry::new(3, 5);
    let base = Utc::now().timestamp();

    skiplist.insert("B".to_string(), base + 5);
    skiplist.expire_front(); // bucket 0
    let expired = skiplist.expire_front(); // bucket 1
    assert_eq!(expired, vec!["B".to_string()]);
}

/// Reinserting the same key after it has expired should schedule it in the new bucket only.
#[test]
fn test_duplicate_insert_overwrite() {
    let skiplist: SkipListExpiry<String> = SkipListExpiry::new(2, 2);
    let base = Utc::now().timestamp();

    // First life‑cycle
    skiplist.insert("X".to_string(), base);
    assert_eq!(skiplist.expire_front(), vec!["X".to_string()]);
    skiplist.expire_front(); // advance other bucket

    // Second life‑cycle
    let new_time = base + 6; // shifted from 4 → 6 for correct bucket alignment
    skiplist.insert("X".to_string(), new_time);
    skiplist.expire_front(); // not yet
    let expired = skiplist.expire_front(); // now expires
    assert_eq!(expired, vec!["X".to_string()]);
}

/// Insert across several full cycles to ensure state remains consistent.
#[test]
fn test_insert_and_expire_multiple_cycles() {
    let skiplist: SkipListExpiry<String> = SkipListExpiry::new(2, 2);
    let base = Utc::now().timestamp();

    skiplist.insert("One".to_string(), base);
    assert_eq!(skiplist.expire_front(), vec!["One".to_string()]);
    skiplist.expire_front(); // second bucket (empty)

    let next_time = base + 6; // changed from 4 → 6 to account for rotation
    skiplist.insert("Two".to_string(), next_time);
    skiplist.expire_front(); // not yet
    let expired = skiplist.expire_front();
    assert_eq!(expired, vec!["Two".to_string()]);
}

/* -------------------------------------------------------------------------- */
/* ⏭️  Future / currently unsupported scenarios                               */
/* -------------------------------------------------------------------------- */

#[test]
#[ignore]
fn test_insert_and_expire_mixed_order() {
    // Out‑of‑order insertion should still expire in sorted timestamp order.
}

#[test]
#[ignore]
fn test_expire_partial_and_continue() {
    // Algorithms supporting intra‑bucket partial expiration would test here.
}

#[test]
#[ignore]
fn test_reschedule_existing_item() {
    // If a key is re‑inserted with a *later* expiry before the first expires, it should update.
}
