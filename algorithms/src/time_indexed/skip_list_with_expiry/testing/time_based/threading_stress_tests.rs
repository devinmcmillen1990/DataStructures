use crate::time_indexed::skip_list_with_expiry::traits::TimeBasedExpiry;

pub fn threading_stress_test_concurrent_insertions<E>(mut expiry: E)
where
    E: TimeBasedExpiry<String>,
{
    // TODO: write out implementation
}

pub fn threading_stress_test_concurrent_expiry_behavior<E>(mut expiry: E)
where
    E: TimeBasedExpiry<String>,
{
    // TODO: write out implementation
}

pub fn threading_stress_test_concurrent_mixed_read_write<E>(mut expiry: E)
where
    E: TimeBasedExpiry<String>,
{
    // TODO: write out implementation
}
