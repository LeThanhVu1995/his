pub mod health;
pub mod tests;
pub mod specimens;
pub mod results;

pub use health::healthz;
pub use tests::{list_tests, create_test, update_test};
pub use specimens::{list_specimens, create_specimen, collect_specimen, receive_specimen, reject_specimen};
pub use results::{list_results, create_result, enter_values, verify_result, release_result};
