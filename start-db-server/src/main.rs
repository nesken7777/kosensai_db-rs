#![windows_subsystem = "windows"]

use std::{
    error::Error,
    process::{Child, Command},
    sync::OnceLock,
};

use windows::{
    core::*,
    Win32::{
        Foundation::{BOOL, HWND, LPARAM, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            DialogBoxParamW, EndDialog, GetDlgItemInt, SetDlgItemTextW, WM_COMMAND, WM_INITDIALOG,
        },
    },
};
const DIALOG: usize = 4000;
const PORT_NUM_INPUT: i32 = 5001;
const DIALOG_TEXT: i32 = 5004;
const SERVER_START: usize = 5005;
const SERVER_STOP: usize = 5006;
static G_HDLG: OnceLock<HWND> = OnceLock::new();
static mut G_CHILD: OnceLock<Child> = OnceLock::new();
fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleW(None)?;
        DialogBoxParamW(
            instance,
            PCWSTR::from_raw(DIALOG as *const u16),
            HWND(0),
            Some(dlg_proc),
            LPARAM(0),
        );
        Ok(())
    }
}

unsafe extern "system" fn dlg_proc(
    window_handle: HWND,
    message: u32,
    wparam: WPARAM,
    _: LPARAM,
) -> isize {
    match message {
        WM_INITDIALOG => {
            G_HDLG.get_or_init(|| window_handle);
            0
        }
        WM_COMMAND => match wparam.0 & 0xffff {
            2 => {
                let _ = close_server();
                EndDialog(window_handle, 2).map_or_else(|_| 0, |_| 0)
            }
            SERVER_START => start_server().map_or_else(
                |e| {
                    let _ = SetDlgItemTextW(
                        *G_HDLG.get().unwrap(),
                        DIALOG_TEXT,
                        PCWSTR::from_raw(
                            HSTRING::from(format!("サーバー起動できてませんよ: {}", e)).as_ptr(),
                        ),
                    );
                    0
                },
                |_| 1,
            ),
            SERVER_STOP => close_server().map_or_else(
                |e| {
                    let _ = SetDlgItemTextW(
                        *G_HDLG.get().unwrap(),
                        DIALOG_TEXT,
                        PCWSTR::from_raw(
                            HSTRING::from(format!("サーバー停止できなかった: {}", e)).as_ptr(),
                        ),
                    );
                    0
                },
                |_| 1,
            ),
            _ => 0,
        },
        _ => 0,
    }
}

fn start_server() -> std::result::Result<(), Box<dyn Error>> {
    let mut port_check = BOOL::default();
    let port_num = unsafe {
        GetDlgItemInt(
            *G_HDLG.get().unwrap(),
            PORT_NUM_INPUT,
            Some(&mut port_check),
            false,
        )
    };
    // 入力されてなかったり有効なポート番号じゃなかったりしたら80番になるから気をつけてな～(^-^)/
    let port_num = port_check.ok().map_or_else(
        |_| {
            unsafe {
                let _ = SetDlgItemTextW(
                    *G_HDLG.get().unwrap(),
                    DIALOG_TEXT,
                    PCWSTR::from_raw(
                        HSTRING::from("入力されてないっぽいのでデフォルトの80番ポートで起動するよ")
                            .as_ptr(),
                    ),
                );
            }
            80
        },
        |_| {
            u16::try_from(port_num).unwrap_or_else(|e| {
                unsafe {
                    let _ = SetDlgItemTextW(
                        *G_HDLG.get().unwrap(),
                        DIALOG_TEXT,
                        PCWSTR::from_raw(
                            HSTRING::from(format!(
                                "入力値エラーなので80番ポートで起動するよ: {}",
                                e
                            ))
                            .as_ptr(),
                        ),
                    );
                }
                80
            })
        },
    );
    if unsafe { G_CHILD.get().is_some() } {
        return Err(Box::<dyn Error + Send + Sync>::from(
            "子プロセスがすでに起動しているらしい",
        ));
    }
    let child = Command::new("./kosensai_db")
        .arg(port_num.to_string())
        .spawn()?;
    unsafe {
        G_CHILD.get_or_init(|| child);
    }
    Ok(())
}

fn close_server() -> std::result::Result<(), String> {
    unsafe {
        G_CHILD.take().map_or(
            Err(String::from("子プロセスがいない")),
            |mut child| {
                child
                    .kill()
                    .map_err(|_| String::from("子プロセスをkillできなかった"))
            },
        )?;
    }
    Ok(())
}
