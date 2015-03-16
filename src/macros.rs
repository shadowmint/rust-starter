#[macro_export]
macro_rules! fmt(($token:expr) => (format!("{:?}", $token)));

