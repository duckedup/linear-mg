use linear_mg::config::auth::resolve_api_key;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_cli_key_takes_precedence() {
    let _lock = ENV_LOCK.lock().unwrap();
    unsafe { std::env::set_var("LINEAR_API_KEY", "env-key") };
    let result = resolve_api_key(Some("cli-key"), Some("config-key"));
    assert_eq!(result.unwrap(), "cli-key");
    unsafe { std::env::remove_var("LINEAR_API_KEY") };
}

#[test]
fn test_env_var_takes_precedence_over_config() {
    let _lock = ENV_LOCK.lock().unwrap();
    unsafe { std::env::set_var("LINEAR_API_KEY", "env-key") };
    let result = resolve_api_key(None, Some("config-key"));
    assert_eq!(result.unwrap(), "env-key");
    unsafe { std::env::remove_var("LINEAR_API_KEY") };
}

#[test]
fn test_config_key_used_as_fallback() {
    let _lock = ENV_LOCK.lock().unwrap();
    unsafe { std::env::remove_var("LINEAR_API_KEY") };
    let result = resolve_api_key(None, Some("config-key"));
    assert_eq!(result.unwrap(), "config-key");
}

#[test]
fn test_no_key_returns_error() {
    let _lock = ENV_LOCK.lock().unwrap();
    unsafe { std::env::remove_var("LINEAR_API_KEY") };
    let result = resolve_api_key(None, None);
    assert!(result.is_err());
}
