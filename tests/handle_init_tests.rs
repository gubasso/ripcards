use std::{fs::read_to_string, path::Path};

use anyhow::Result;
use cmd_lib::run_fun;
use ripcards::{
    config::Config, handlers::handle_init, msgs::GIT_COMMIT_MSG_RIPC_INIT,
    utils::set_current_directory,
};
use tempfile::tempdir;

#[test]
fn test_handle_init() -> Result<()> {
    let temp_dir = tempdir()?;
    set_current_directory(&temp_dir)?;
    handle_init()?;
    let gitkeep_path_str = "ripc/sessions/.gitkeep";
    let config_path_str = "ripc/config.toml";

    let config_content = read_to_string(config_path_str)?;
    let config: Config = toml::from_str(&config_content)?;
    assert_eq!(config, Config::default());

    assert!(
        Path::new(".git").is_dir(),
        "Git repository must be initialized."
    );
    assert!(Path::new(gitkeep_path_str).is_file());
    assert!(Path::new(config_path_str).is_file());

    let out_git_status = run_fun!(git status --porcelain)?;
    assert!(!out_git_status.contains(gitkeep_path_str));
    assert!(!out_git_status.contains(config_path_str));

    let out_git_log = run_fun!(git log --oneline)?;
    assert!(out_git_log.contains(GIT_COMMIT_MSG_RIPC_INIT));

    Ok(())
}
