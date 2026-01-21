import React, { useState } from 'react';
import { Box, TextField, Button, Typography, CircularProgress, Alert } from '@mui/material';
import { useGetAccountInfo, useGetLoginInfo } from '@multiversx/sdk-dapp/hooks';
import { createParticipant } from '../services/multiversx.services';

const CreateParticipant = () => {
  const [username, setUsername] = useState('');
  const [loading, setLoading] = useState(false);
  const [success, setSuccess] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const { account } = useGetAccountInfo();
  const { isLoggedIn } = useGetLoginInfo();

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    if (!isLoggedIn) {
      setError('Please connect your wallet first.');
      return;
    }
    if (!username.trim()) {
      setError('Username cannot be empty.');
      return;
    }

    setLoading(true);
    setError(null);
    setSuccess(null);

    try {
      // The `provider` is typically obtained from `useGet and `useTrackTransactionStatus`
      // For simplicity in this example, we'll assume a direct call or placeholder for `provider` if needed by `createParticipant`
      // In a real dApp, you'd integrate with the dApp provider for signing transactions.
      // For now, we are passing `null` for provider, assuming the `createParticipant` function
      // is designed to handle this or has its own way to sign.
      // A more robust solution would involve using `sendTransactions` from `@multiversx/sdk-dapp/services`
      // and getting the signer from the dApp context.

      // Placeholder for `provider` until actual integration is done.
      // In a real scenario, you'd use a hook like `useTrackTransactionStatus` or `sendTransactions`
      // which would handle the signing.
      await createParticipant(username, account, null); // `null` is a placeholder for the provider
      setSuccess('Participant created successfully!');
      setUsername('');
    } catch (err: any) {
      console.error('Failed to create participant:', err);
      setError(err.message || 'Failed to create participant. Please try again.');
    } finally {
      setLoading(false);
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

      {success && (
        <Alert severity="success" sx={{ width: '100%' }}>
          {success}
        </Alert>
      )}

      {error && (
        <Alert severity="error" sx={{ width: '100%' }}>
          {error}
        </Alert>
      )}

      <TextField
        label="Username"
        variant="outlined"
        fullWidth
        value={username}
        onChange={(e) => setUsername(e.target.value)}
        disabled={loading || !isLoggedIn}
        required
      />
      <Button
        type="submit"
        variant="contained"
        color="primary"
        fullWidth
        disabled={loading || !isLoggedIn}
      >
        {loading ? <CircularProgress size={24} /> : 'Register Participant'}
      </Button>
    </Box>
  );
};

export default CreateParticipant;
