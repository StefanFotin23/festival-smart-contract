import React, { useState } from 'react';
import { Box, Button, Typography, Grid } from '@mui/material';
import { useGetAccountInfo, useTrackTransactionStatus, sendTransactions } from '@multiversx/sdk-dapp';
import { Transaction } from '@multiversx/sdk-core';
import { TransactionFactory } from '../services/TransactionService';

export const Actions = () => {
  const { address } = useGetAccountInfo();
  const [sessionId, setSessionId] = useState<string | null>(null);

  const { isLoading, isSuccessful, isFailed, error } = useTrackTransactionStatus({ sessionId });

  const handleTx = async (tx: Transaction, displayInfo: string) => {
    try {
      const { sessionId: newSessionId, error } = await sendTransactions({
        transactions: [tx],
        transactionsDisplayInfo: {
          processingMessage: `Processing: ${displayInfo}`,
          errorMessage: `Error: ${displayInfo}`,
          successMessage: `Success: ${displayInfo}`,
        },
        redirectAfterSign: false,
      });

      if (error) throw new Error(error);
      setSessionId(newSessionId);

    } catch (err: any) {
      console.error(`Failed to process transaction: ${displayInfo}`, err);
      alert(err.message || `Failed: ${displayInfo}`);
    }
  };

  // --- Handlers ---
  const handleBuyTicket = () => {
    const tx = TransactionFactory.buyTicket(address, 1, 'Full Pass Early Bird', 0.1);
    handleTx(tx, "Buy Ticket");
  };

  const handleCheckIn = () => {
    const nonce = prompt("Enter ticket nonce to check in:", "1");
    if (nonce) {
      const tx = TransactionFactory.checkIn(address, parseInt(nonce));
      handleTx(tx, "Check In");
    }
  };

  const handleCheckOut = () => {
    const festivalId = prompt("Enter festival ID to check out from:", "1");
    if (festivalId) {
      const tx = TransactionFactory.checkOut(address, parseInt(festivalId));
      handleTx(tx, "Check Out");
    }
  };
  
  const handleSellTicket = () => {
    const nonce = prompt("Enter ticket nonce to sell:", "1");
    const price = prompt("Enter price in EGLD:", "0.5");
    if (nonce && price) {
      const tx = TransactionFactory.sellTicket(address, parseInt(nonce), parseFloat(price));
      handleTx(tx, "Put Ticket for Sale");
    }
  };

  const handleClaimPoints = () => {
    const festivalId = prompt("Enter festival ID:", "1");
    const eventIndex = prompt("Enter flash event index:", "0");
    if (festivalId && eventIndex) {
      const tx = TransactionFactory.claimFlashPoints(address, parseInt(festivalId), parseInt(eventIndex));
      handleTx(tx, "Claim Flash Points");
    }
  };

  const handleBuyResale = () => {
    const nonce = prompt("Enter resale ticket nonce to buy:", "1");
    const price = prompt("Enter price in EGLD:", "0.5");
    if (nonce && price) {
      const tx = TransactionFactory.buyResaleTicket(address, parseInt(nonce), parseFloat(price));
      handleTx(tx, "Buy Resale Ticket");
    }
  };

  // --- UI ---
  return (
    <Box sx={{ mt: 4 }}>
      <Typography variant="h4" gutterBottom>
        Festival Actions
      </Typography>
      
      {isLoading && <Typography>Transaction in progress...</Typography>}
      {isSuccessful && <Typography color="success.main">Transaction successful!</Typography>}
      {isFailed && <Typography color="error.main">Transaction failed: {error}</Typography>}
      
      <Grid container spacing={2} sx={{ mt: 2 }}>
        <Grid item xs={12} sm={4}>
          <Button variant="contained" fullWidth onClick={handleBuyTicket}>Buy Ticket (0.1 EGLD)</Button>
        </Grid>
        <Grid item xs={12} sm={4}>
          <Button variant="contained" fullWidth onClick={handleCheckIn}>Check In</Button>
        </Grid>
        <Grid item xs={12} sm={4}>
          <Button variant="contained" fullWidth onClick={handleCheckOut}>Check Out</Button>
        </Grid>
        <Grid item xs={12} sm={4}>
          <Button variant="contained" fullWidth onClick={handleSellTicket}>Sell Ticket</Button>
        </Grid>
        <Grid item xs={12} sm={4}>
          <Button variant="contained" fullWidth onClick={handleClaimPoints}>Claim Flash Points</Button>
        </Grid>
        <Grid item xs={12} sm={4}>
          <Button variant="contained" fullWidth onClick={handleBuyResale}>Buy Resale Ticket</Button>
        </Grid>
      </Grid>
    </Box>
  );
};
