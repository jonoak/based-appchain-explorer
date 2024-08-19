import React, { useState, useEffect } from 'react';
import axios from 'axios';

const App = () => {
  const [l2s, setL2s] = useState([]);
  const [name, setName] = useState('');
  const [config, setConfig] = useState('');

  useEffect(() => {
    axios.get('/api/v1/l2s').then((response) => setL2s(response.data));
  }, []);

  const createL2 = () => {
    axios.post('/api/v1/l2s', { id: Date.now().toString(), name, config }).then(() => {
      axios.get('/api/v1/l2s').then((response) => setL2s(response.data));
    });
  };

  return (
    <div>
      <h1>Based Appchain Explorer</h1>
      <div>
        <input value={name} onChange={(e) => setName(e.target.value)} placeholder="L2 Name" />
        <input value={config} onChange={(e) => setConfig(e.target.value)} placeholder="L2 Config" />
        <button onClick={createL2}>Create L2</button>
      </div>
      <ul>
        {l2s.map((l2) => (
          <li key={l2.id}>{l2.name}</li>
        ))}
      </ul>
    </div>
  );
};

export default App;
