#![windows_subsystem = "windows"]

use std::env;
use std::process::Command;
use clipboard::{ClipboardContext, ClipboardProvider};
use winapi::um::winuser::{GetAsyncKeyState, VK_LCONTROL, VK_RCONTROL, VK_LSHIFT, VK_RSHIFT};

fn main() {
    hk_loop();
}
fn hk_loop() {
    const DELAY: u32 = 5;
    let mut cmd: Option<HKCommand> = None;
    let mut run: Option<HKCommand> = None;
    let mut timeout = 0;
    loop {
        unsafe {
            if is_first_row() {
                if let Some(hk) = first_row_command() {
                    if cmd != Some(hk) {
                        cmd = Some(hk);
                    }
                }
            }
            if is_second_row() {
                if let Some(hk) = second_row_command() {
                    if cmd != Some(hk) {
                        cmd = Some(hk);
                    }
                }
            }
            if is_third_row() {
                if let Some(hk) = third_row_command() {
                    if cmd != Some(hk) {
                        cmd = Some(hk);
                    }
                }
            }
        }
        if cmd.is_some(){
            timeout += 1;
            if run != cmd{
                run = cmd.clone();
                run_command(cmd.unwrap());
            }
        }
        if timeout >= DELAY {
            timeout = 0;
            cmd = None;
            run = None;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
unsafe fn is_first_row() -> bool {
    GetAsyncKeyState(VK_LCONTROL) < 0
        && GetAsyncKeyState(VK_RCONTROL) < 0
        && GetAsyncKeyState(VK_LSHIFT) < 0
        && GetAsyncKeyState(VK_RSHIFT) < 0
}
unsafe fn is_second_row() -> bool {
    GetAsyncKeyState(VK_LCONTROL) < 0
        && GetAsyncKeyState(VK_RCONTROL) < 0
        && GetAsyncKeyState(VK_LSHIFT) < 0
}
unsafe fn is_third_row() -> bool {
    GetAsyncKeyState(VK_LCONTROL) < 0
        && GetAsyncKeyState(VK_LSHIFT) < 0
}
#[derive(PartialEq, Clone, Copy)]
enum HKCommand{
    RunPowerShell,
    RunPowerShellYtDlp,
    RunPowerShellYtDlpFirefox,
    RunPowerShellYtDlpFirefoxOnlyText,
}
unsafe fn first_row_command() -> Option<HKCommand> {
    if GetAsyncKeyState(0x31) < 0 {
        Some(HKCommand::RunPowerShell)
    } else if GetAsyncKeyState(0x32) < 0 {
        Some(HKCommand::RunPowerShellYtDlp)
    } else if GetAsyncKeyState(0x33) < 0 {
        Some(HKCommand::RunPowerShellYtDlpFirefox)
    } else if GetAsyncKeyState(0x34) < 0 {
        Some(HKCommand::RunPowerShellYtDlpFirefoxOnlyText)
    } else{
        None
    }
}
unsafe fn second_row_command() -> Option<HKCommand> {
    None
}
unsafe fn third_row_command() -> Option<HKCommand> {
    None
}
fn run_command(cmd: HKCommand) {
    match cmd {
        HKCommand::RunPowerShell => {
            run_powershell();
        },
        HKCommand::RunPowerShellYtDlp => {
            run_yt_dlp();
        },
        HKCommand::RunPowerShellYtDlpFirefox => {
            run_yt_dlp_firefox();
        },
        HKCommand::RunPowerShellYtDlpFirefoxOnlyText => {
            run_yt_dlp_firefox_only_text();
        }
    }
}
fn run_powershell(){
    if let Err(err) =
        Command::new("powershell.exe")
            .current_dir(get_desktop_path())
            .spawn(){
        eprintln!("{}", err);
    }
}
fn run_yt_dlp(){
    if let Err(err) =
        Command::new("yt-dlp")
            .args(&[
                &get_clipboard_text(),
            ])
            .current_dir(get_desktop_path())
            .spawn()
    {
        eprintln!("{}", err);
    }
}
fn run_yt_dlp_firefox(){
    if let Err(err) =
        Command::new("yt-dlp")
            .args(&[
                "--cookies-from-browser firefox",
                &get_clipboard_text(),
            ])
            .current_dir(get_desktop_path())
            .spawn()
    {
        eprintln!("{}", err);
    }
}
fn run_yt_dlp_firefox_only_text(){
    if let Err(err) =
        Command::new("yt-dlp")
            .args(&[
                "--cookies-from-browser firefox",
                "--write-auto-sub",
                "--sub-lang ru",
                "--skip-download",
                &get_clipboard_text(),
            ])
            .current_dir(get_desktop_path())
            .spawn()
    {
        eprintln!("{}", err);
    }
}

fn get_desktop_path() -> String {
    format!("{}\\Desktop",  env::var("USERPROFILE").unwrap_or(String::from("")))
}
fn get_clipboard_text() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.get_contents().unwrap()
}