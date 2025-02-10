import React, { useState } from 'react';
import axios from 'axios';

function App() {
  const [file, setFile] = useState(null);
  const [ticket, setTicket] = useState('');
  const [receivedFile, setReceivedFile] = useState(null);

  const handleUpload = async () => {
    const formData = new FormData();
    formData.append('file', file);

    const response = await axios.post('/api/send', formData);
    setTicket(response.data.ticket);
  };

  const handleDownload = async () => {
    const response = await axios.post('/api/receive', { ticket });
    
    if (response.data.error) {
      alert('Failed to retrieve file');
      return;
    }

    const blobUrl = URL.createObjectURL(new Blob([response.data]));
    setReceivedFile(blobUrl);
  };

  return (
    <div>
      <h1>P2P File Transfer</h1>
      
      <div>
        <h2>Send File</h2>
        <input type="file" onChange={(e) => setFile(e.target.files[0])} />
        <button onClick={handleUpload}>Upload</button>
        {ticket && <p>Share this ticket: {ticket}</p>}
      </div>

      <div>
        <h2>Receive File</h2>
        <input 
          type="text" 
          placeholder="Enter ticket" 
          value={ticket} 
          onChange={(e) => setTicket(e.target.value)} 
        />
        <button onClick={handleDownload}>Download</button>
      </div>

      {receivedFile && (
        <div>
          <h3>Received File:</h3>
          <a href={receivedFile} download="file">Download File</a>
        </div>
      )}
    </div>
  );
}

export default App;
