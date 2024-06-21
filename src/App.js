import React, {useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event'

function App() {
  const[ip, setIp] = useState('');
  const[response, setResponse ] = useState('');
  const [logs, setLogs] = useState([]);

  useEffect(() => {
    const unlisten = listen('log', (event) => {
      setLogs((prevLogs) => [...prevLogs, event.payload]);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  },[]);

  const handleConect = async () => {
    try {
      const result = await invoke('connect_to_server', {ip});
      setResponse(result);
    }catch(e){
      setResponse(`Erro: ${e}`);
    }
  };

  return (
    <div className="App">
      <h1>Conectar ao clp</h1>
      <input
        type="text"
        placeholder='Digite o ip do CLP'
        value={ip}
        onChange={(e) => setIp(e.target.value)}
      />
      <button onClick={handleConect}>Conectar</button>
      <p>{response}</p>
      <div className='logs'>
        <h2>Logs</h2>
        <div className='log-messages'>
          {logs.map((log, index) => (
            <p key={index}>{log}</p>
          ))}
        </div>
      </div>
    </div>
  );
}

export default App;