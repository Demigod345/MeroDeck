import React from 'react';
import { Routes, Route, BrowserRouter } from 'react-router-dom';
import HomePage from './pages/home';
import Play from './pages/play/play';
import SetupPage from './pages/setup';
import Authenticate from './pages/login/Authenticate'; 
import { AccessTokenWrapper } from '@calimero-is-near/calimero-p2p-sdk';
import { getNodeUrl } from './utils/node';

export default function App() {
  return (
    <AccessTokenWrapper getNodeUrl={getNodeUrl}>
      <BrowserRouter basename="/demo-blockchain-integrations/">
        <Routes>
          <Route path="/" element={<SetupPage />} />
          <Route path="/auth" element={<Authenticate />} />
          <Route path="/home" element={<HomePage />} />
          <Route path="/play" element={<Play />} />
        </Routes>
      </BrowserRouter>
    </AccessTokenWrapper>
  );
}
