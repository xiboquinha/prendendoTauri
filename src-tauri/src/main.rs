#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::command;
use tokio::io::{self, AsyncReadExt};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::TcpStream;
use chrono::Local;
use serde::Serialize;
use tokio::time::{self, Duration};
use rusqlite::{params, Connection};
use crossbeam::channel::{unbounded, Sender};

#[derive(Serialize)]
struct ServerMessage {
  timestamp: String,
  message: String,
}

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![connect_to_server])
    .run(tauri::generate_context!())
    .expect("ja deeu erro papai KKKKKKKKKKKKKKKK")
}

#[command]
async fn connect_to_server(ip: String, window: tauri::Window) -> Result<String, String> {
  let (tx, rx) = unbounded();

  std::thread::spawn(move || {
    let conn = Connection::open("msgsServer.db").expect("erro ao abrir db");
    conn.execute(
      "CREATE TABLE IF NOT EXISTS messages(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        message TEXT NOT NULL,
        timestamp DATE NOT NULL
      )",
      [],
    ).expect("erro ao criar db");

    while let Ok((timestamp, message)) = rx.recv() {
      conn.execute(
        "INSERT INTO messages (timestamp, message) VALUES (?1, ?2)",
        params![timestamp, message],
      ).expect("erro ao inserir dados no db");
    }
  });

  match TcpStream::connect(&ip).await {
    Ok(conn) => {
      window.emit("log", format!("conecato ao {}", ip)).unwrap();
      let (reader, _writer) = conn.into_split();
      if let Err(e) = read_from_server(reader, window.clone(), tx).await {
        return Err(format!("erro ao ler do server: {}", e));
      }
      Ok("conexao estabelecida".into())
    }
    Err(e) => Err(format!("erro ao conectar: {}", e)),
  }
}

async fn read_from_server(mut reader: OwnedReadHalf, window: tauri::Window, tx: Sender<(String, String)>) -> io::Result<()> {
  let mut buffer = vec![0; 2048];

  loop {
    match reader.read(&mut buffer).await {
      Ok(0) => {
        window.emit("log", "conexao fechada pelo servidor".to_string()).unwrap();
        break;
      },
      Ok(n) => {
        let msg = String::from_utf8_lossy(&buffer[..n]).into_owned();
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let server_message = ServerMessage { timestamp: timestamp.clone(), message: msg.clone() };

        tx.send((timestamp.clone(), msg.clone())).expect("erro ao enviar dados pelo canal");

        window.emit("log", serde_json::to_string(&server_message).unwrap()).unwrap();
      },
      Err(e) => {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let server_message = ServerMessage { timestamp, message: format!("erro ao ler: {}", e) };
        window.emit("log", serde_json::to_string(&server_message).unwrap()).unwrap();
        return Err(e.into());
      }
    }
  }
  Ok(())
}
