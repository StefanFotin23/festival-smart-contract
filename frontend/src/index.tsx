import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter as Router } from 'react-router-dom';
import { DAppProvider } from '@multiversx/sdk-dapp';
import { StyledEngineProvider, ThemeProvider, createTheme, CssBaseline } from '@mui/material';

import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import { apiNetwork, walletConnectV2ProjectId } from './config/network';

const darkTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#90caf9',
    },
    secondary: {
      main: '#f48fb1',
    },
    background: {
      default: '#121212',
      paper: '#1d1d1d',
    },
  },
});

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
  <React.StrictMode>
    <StyledEngineProvider injectFirst>
      <ThemeProvider theme={darkTheme}>
        <CssBaseline />
        <Router>
          <DAppProvider
            environment={apiNetwork.id}
            customNetworkConfig={{
              name: apiNetwork.id,
              type: apiNetwork.id,
              apiAddress: apiNetwork.apiAddress,
              walletAddress: apiNetwork.walletAddress,
              explorerAddress: apiNetwork.explorerAddress,
              chainId: apiNetwork.chainId,
            }}
            dappConfig={{
              walletConnectV2ProjectId,
            }}
          >
            <App />
          </DAppProvider>
        </Router>
      </ThemeProvider>
    </StyledEngineProvider>
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
