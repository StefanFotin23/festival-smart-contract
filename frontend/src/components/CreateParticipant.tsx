import React, { useState } from 'react';
import { Box, TextField, Button, Typography, CircularProgress, Alert } from '@mui/material';
import { useGetAccountInfo, useGetLoginInfo, useTrackTransactionStatus, sendTransactions } from '@multiversx/sdk-dapp';
import { TransactionFactory } from '../services/TransactionService';

const CreateParticipant = () => {
  const [username, setUsername] = useState('');
  const [sessionId, setSessionId] = useState<string | null>(null);

  const { address } = useGetAccountInfo();
  const { isLoggedIn } = useGetLoginInfo();

  const {
    isLoading,
    isSuccessful,
    isFailed,
    error: transactionError,
  } = useTrackTransactionStatus({ sessionId });

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    if (!isLoggedIn) {
      alert('Please connect your wallet first.');
      return;
    }
    if (!username.trim()) {
      alert('Username cannot be empty.');
      return;
    }

    try {
      const transaction = TransactionFactory.createParticipant(address, username);
      
      const { sessionId: newSessionId, error } = await sendTransactions({
        transactions: [transaction],
        transactionsDisplayInfo: {
          processingMessage: 'Processing registration...',
          errorMessage: 'An error has occurred',
          successMessage: 'Participant registered successfully!',
        },
        redirectAfterSign: false,
      });

      if (error) {
        throw new Error(error);
      }

      setSessionId(newSessionId);
      setUsername('');

    } catch (err: any) {
      console.error('Failed to create participant:', err);
      alert(err.message || 'Failed to register. Please try again.');
    }
  };

  return (
    <Box
      component="form"
      onSubmit={handleSubmit}
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        gap: 2,
        mt: 4,
        p: 3,
        border: '1px solid grey',
        borderRadius: '8px',
        maxWidth: '400px',
        mx: 'auto',
      }}
    >
      <Typography variant="h5" component="h1" gutterBottom>
        Create Participant Profile
      </Typography>

      {!isLoggedIn && (
        <Alert severity="warning" sx={{ width: '100%' }}>
          Please connect your wallet to create a participant profile.
        </Alert>
      )}

      {isSuccessful && (
        <Alert severity="success" sx={{ width: '100%' }}>
          Registration successful!
        </Alert>
      )}

      {isFailed && (
        <Alert severity="error" sx={{ width: '100%' }}>
          Registration failed: {transactionError}
        </Alert>
      )}

      {isLoading && (
        <Box sx={{ width: '100%', textAlign: 'center' }}>
          <CircularProgress />
          <Typography>Transaction in progress...</Typography>
        </Box>
      )}

      <TextField
        label="Username"
        variant="outlined"
        fullWidth
        value={username}
        onChange={(e) => setUsername(e.target.value)}
        disabled={isLoading || !isLoggedIn}
        required
      />
      <Button
        type="submit"
        variant="contained"
        color="primary"
        fullWidth
        disabled={isLoading || !isLoggedIn}
      >
        {isLoading ? <CircularProgress size={24} /> : 'Register Participant'}
      </Button>
    </Box>
  );
};

export default CreateParticipant;
