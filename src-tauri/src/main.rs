#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use libp2peasy::{Message, Server, ServerResponse};
use tauri::Manager;
use tokio::sync::{mpsc, oneshot, Mutex};
use tracing::info;

struct AsyncProcInputTx {
    input: Mutex<mpsc::Sender<String>>,
}

#[derive(Debug)]
enum SenderOutput {
    ServerResponse(ServerResponse),
}

fn main() {
    tracing_subscriber::fmt::init();

    // input comes from JS going to Rust
    let (input_sender, input_recvr) = mpsc::channel(1);
    // output comes from Rust going to JS
    let (output_sender, mut output_recvr) = mpsc::channel(1);
    let (server_sendr, server_recvr) = mpsc::channel::<Message<ServerResponse>>(1);

    tauri::Builder::default()
        .manage(AsyncProcInputTx {
            input: Mutex::new(input_sender),
        })
        .invoke_handler(tauri::generate_handler![js2rs])
        .setup(|app| {
            let app_handle = app.handle();

            // example of full cycle through JS to RUST and back up to JS:
            tauri::async_runtime::spawn(async move {
                async_process_model(input_recvr, output_sender, server_sendr).await
            });

            // spawn the WebRTC Server, send the multiaddress back to JS
            tauri::async_runtime::spawn(async move {
                Server::new()
                    .enable_kademlia()
                    .start_with_tokio_executor(server_recvr)
                    .await
                    .unwrap();
            });

            tauri::async_runtime::spawn(async move {
                loop {
                    match output_recvr.recv().await {
                        Some(SenderOutput::ServerResponse(ServerResponse { address })) => {
                            rs2js(
                                "serverMultiaddr",
                                std::str::from_utf8(&address).unwrap().into(),
                                &app_handle,
                            );
                        }
                        // Put the rest of the RPC API here
                        None => {}
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn rs2js<R: tauri::Runtime>(event_label: &str, message: String, manager: &impl Manager<R>) {
    info!(?message, event_label);
    let payload = format!("{message:?}");
    manager.emit_all(event_label, message).unwrap();
}

#[tauri::command]
async fn js2rs(message: String, state: tauri::State<'_, AsyncProcInputTx>) -> Result<(), String> {
    info!(?message, "js2rs");
    // Get these channels from Tauri State
    let input_sender = state.input.lock().await;
    input_sender.send(message).await.map_err(|e| e.to_string())
}

async fn async_process_model(
    mut input_recvr: mpsc::Receiver<String>,
    output_sender: mpsc::Sender<SenderOutput>,
    server_sendr: mpsc::Sender<Message<ServerResponse>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_recvr.recv().await {
        let _output = input;
        // Parse input to determine where the command goes
        // Reply channel sends a ServerResponse back
        let (reply_sender, reply_rcvr) = oneshot::channel::<ServerResponse>();

        let _result = server_sendr
            .send(Message::<ServerResponse> {
                reply: reply_sender,
            })
            .await;

        let reply = reply_rcvr.await?;
        let s: String = std::str::from_utf8(&reply.address).unwrap().into();
        // Rust doesn't support octal character escape sequence
        // For colors, use hexadecimal escape instead, plus a series of semicolon-separated parameters.
        println!("Connect with: \n\x1b[30;1;42m{s}\x1b[0m");

        let _ = output_sender
            .send(SenderOutput::ServerResponse(reply))
            .await;
    }

    Ok(())
}
