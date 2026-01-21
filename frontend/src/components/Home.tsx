import React from 'react';
import { Box, Typography } from '@mui/material';

const Home = () => {
  return (
    <Box sx={{ textAlign: 'center', mt: 4 }}>
      <Typography variant="h3" component="h1" gutterBottom>
        Welcome to the Festival DApp!
      </Typography>
      <Typography variant="h6" color="textSecondary">
        Your portal to amazing festival experiences on the MultiversX blockchain.
      </Typography>
    </Box>
  );
};

export default Home;
