/// Every `#[tauri::command]` registered in `lib.rs`. Declaring an app manifest makes
/// Tauri enforce the ACL on ALL app commands (remote pages otherwise get
/// "Command <x> not allowed by ACL"), so each command must appear here and be granted
/// through an `allow-<name>` permission in `capabilities/*.json`:
/// - `main-remote.json`  -> only what bridge.js calls from the WhatsApp page
/// - `settings.json`     -> what the local settings window calls
/// - `lock.json`         -> what the lock screen calls
/// Per-command `is_remote`/`is_lock_window` guards in `commands.rs` stay as defence in depth.
const COMMANDS: &[&str] = &[
    "notify",
    "set_unread",
    "dlog",
    "toggle_titlebar",
    "titlebar_hidden",
    "start_drag",
    "toggle_maximize",
    "window_minimize",
    "window_close",
    "switch_account",
    "open_settings_from_page",
    "get_settings",
    "set_settings",
    "open_settings",
    "list_accounts",
    "add_account",
    "remove_account",
    "rename_account",
    "open_account",
    "get_lock_status",
    "set_app_lock_password",
    "change_app_lock_password",
    "disable_app_lock",
    "set_app_lock_options",
    "set_biometric_enabled",
    "lock_app",
    "unlock_password",
    "unlock_biometric",
    "reset_app_lock",
];

fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .app_manifest(tauri_build::AppManifest::new().commands(COMMANDS)),
    )
    .expect("failed to run tauri-build");
}
