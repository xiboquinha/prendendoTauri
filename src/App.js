import React, {useEffect, useState, useRef } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';
import { Chart, registerables } from 'chart.js/auto';
import 'chartjs-adapter-date-fns';
import './App.css';

Chart.register(...registerables);

function App() {
  const[ip, setIp] = useState('');
  const[response, setResponse ] = useState('');
  const [logs, setLogs] = useState([]);
  const chartRef = useRef(null);
  const chartInstance = useRef(null);

  useEffect(() => {
    const unlisten = listen('log', (event) => {
      const log = JSON.parse(event.payload);
      log.message = Number(log.message);
      setLogs((prevLogs) => [...prevLogs, log]);
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

  useEffect(() => {
    if (chartRef.current) {
      const ctx = chartRef.current.getContext('2d');

      if (chartInstance.current){
        chartInstance.current.destroy();
      }
      chartInstance.current = new Chart(ctx, {
        type: 'line', 
        data: {
          labels: logs.map(log => log.timestamp),
          datasets:[{
            label: 'Mensagens do Servidor',
            data: logs.map(log => log.message),
            borderColor: 'rgba(50, 0, 250, 1)',
            borderWidth: 1,
          }],
        },
        options:{
          scales:{
            x:{
              type: 'time',
              time:{     
                unit: 'second',
                displayFormats:{
                  second: 'YYYY-MM-DD HH:mm:ss'
              }
            }
          }
        }
      }
    });  
  }
},[logs]);

  return (
    <div className="App">
      <div className='tittle-bar'>
        <div className='tittle'>Supervisao</div>
        <div className='tittle-bar-buttons'>
          <button onClick={() => appWindow.minimize()}>-</button>
          <button onClick={() => appWindow.toggleMaximize()}>⬜</button>
          <button onClick={() => appWindow.close()}>X</button>
        </div>
      </div>
      <h1>Conectar ao clp</h1>
      <input
        type="text"
        placeholder='Digite o ip do CLP'
        value={ip}
        onChange={(e) => setIp(e.target.value)}
      />
      <button onClick={handleConect}>Conectar</button>
      <p>{response}</p>
      <canvas id = "dash" ref={chartRef} width= "400" height= "200"></canvas>
    </div>
  );
}

export default App;