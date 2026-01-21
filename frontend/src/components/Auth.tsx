import React from 'react';
import {
  ExtensionLoginButton,
  WebWalletLoginButton,
  LedgerLoginButton,
  WalletConnectLoginButton
} from '@multiversx/sdk-dapp/UI';
import { Box, Typography, Button } from '@mui/material';
import { useNavigate } from 'react-router-dom';

const Auth = () => {
  const navigate = useNavigate();
  const onSuccess = () => {
    navigate('/');
  };

  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        minHeight: '60vh',
        gap: 2,
      }}
    >
      <Typography variant="h5" color="textPrimary" gutterBottom>
        Connect your MultiversX Wallet
      </Typography>
      <ExtensionLoginButton
        callbackRoute="/"
        loginButtonText="Extension"
        onSuccess={onSuccess}
      />
      <WebWalletLoginButton
        callbackRoute="/"
        loginButtonText="Web Wallet"
        onSuccess={onSuccess}
      />
      <LedgerLoginButton
        callbackRoute="/"
        loginButtonText="Ledger"
        onSuccess={onSuccess}
      />
      <WalletConnectLoginButton
        callbackRoute="/"
        loginButtonText="WalletConnect"
        onSuccess={onSuccess}
      />
      <Button variant="text" onClick={() => navigate('/')}>
        Back to Home
      </Button>
    </Box>
  );
};

export default Auth;
