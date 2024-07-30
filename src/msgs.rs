pub mod error;

pub const GIT_COMMIT_MSG_RIPC_INIT: &str = "RipCards project initialized.";
pub const GIT_COMMIT_MSG_RIPC_NEW: &str = "RipCard project initialized.";

pub fn git_commit_msg_ripc_new(card_id: &str) -> String {
    format!("Card created with id: {}.", card_id)
}
