#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fmt::format;

use tauri::command;
use tokio::io::{self, AsyncReadExt};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::TcpStream;


#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![connect_to_server])
    .run(tauri::generate_context!())
    .expect("ja deeu erro papai KKKKKKKKKKKKKKKK")
}

#[command]
async fn connect_to_server(ip: String, window: tauri::Window) -> Result<String, String> {
  match TcpStream::connect(&ip).await {
    Ok(conn) => {
      window.emit("log", format!("conecato ao {}", ip)).unwrap();
      let (reader, _writer) = conn.into_split();
      if let Err(e) = read_from_server(reader, window.clone()).await {
        return Err(format!("erro ao ler do server: {}", e));
      }
      Ok("conexao estabelecida".into())
    }
    Err(e) => Err(format!("erro ao conectar: {}", e)),
  }
}

async fn read_from_server(mut reader: OwnedReadHalf, window: tauri::Window) -> io::Result<()> {
  let mut buffer = vec![0;2048];
  loop {
    match reader.read(&mut buffer).await {
      Ok(0) => {
        window.emit("log", "conexao fechada pelo servidor".to_string()).unwrap();
        break;
      },
      Ok(n) => {
        let msg = String::from_utf8_lossy(&buffer[..n]).into_owned();
        window.emit("log", format!("recebido: {}", msg)).unwrap();
      },
      Err(e) => {
        window.emit("log",format!("erro ao ler: {}", e)).unwrap();
        return Err(e.into());
      }
    }
  }
  Ok(())
}
