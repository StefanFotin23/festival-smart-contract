import React from 'react';
import { Routes, Route } from 'react-router-dom';
import { AppBar, Toolbar, Typography, Button, Container, Box } from '@mui/material';
import { useGetLoginInfo } from '@multiversx/sdk-dapp/hooks';
import { logout } from '@multiversx/sdk-dapp/utils';

import './App.css';

// Placeholder Components (will be created later)
import Home from './components/Home';
import Festivals from './components/Festivals';
import CreateParticipant from './components/CreateParticipant';
import Leaderboard from './components/Leaderboard';
import Auth from './components/Auth';

function App() {
  const { isLoggedIn } = useGetLoginInfo();

  const handleLogout = () => {
    logout(`${window.location.origin}`);
  };

  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            Festival DApp
          </Typography>
          <Button color="inherit" onClick={() => window.location.href = '/'}>Home</Button>
          <Button color="inherit" onClick={() => window.location.href = '/festivals'}>Festivals</Button>
          <Button color="inherit" onClick={() => window.location.href = '/create-participant'}>Register</Button>
          <Button color="inherit" onClick={() => window.location.href = '/leaderboard'}>Leaderboard</Button>
          {isLoggedIn ? (
            <Button color="inherit" onClick={handleLogout}>Logout</Button>
          ) : (
            <Button color="inherit" onClick={() => window.location.href = '/unlock'}>Connect Wallet</Button>
          )}
        </Toolbar>
      </AppBar>
      <Container sx={{ mt: 4 }}>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/festivals" element={<Festivals />} />
          <Route path="/create-participant" element={<CreateParticipant />} />
          <Route path="/leaderboard" element={<Leaderboard />} />
          <Route path="/unlock" element={<Auth />} /> {/* Placeholder for login page */}
        </Routes>
      </Container>
    </Box>
  );
}

export default App;
