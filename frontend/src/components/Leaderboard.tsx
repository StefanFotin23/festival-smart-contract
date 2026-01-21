import React, { useEffect, useState } from 'react';
import { Box, Typography, CircularProgress, Alert, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Paper } from '@mui/material';
// import { getLeaderboard } from '../services/multiversx.services'; // Uncomment when actual service is ready

interface LeaderboardEntry {
  address: string;
  username: string;
  score: number;
}

const Leaderboard = () => {
  const [leaderboard, setLeaderboard] = useState<LeaderboardEntry[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchLeaderboard = async () => {
      try {
        // This is a placeholder. In a real application, you would
        // call a smart contract view function or an off-chain API
        // to get the actual leaderboard data.
        // For example: const fetchedLeaderboard = await getLeaderboard();
        const fetchedLeaderboard: LeaderboardEntry[] = [
          { address: 'erd1...user1', username: 'Alice', score: 150 },
          { address: 'erd1...user2', username: 'Bob', score: 120 },
          { address: 'erd1...user3', username: 'Charlie', score: 100 },
        ].sort((a, b) => b.score - a.score);

        setLeaderboard(fetchedLeaderboard);
      } catch (err) {
        console.error('Failed to fetch leaderboard:', err);
        setError('Failed to load leaderboard. This feature requires further smart contract implementation or an indexer.');
      } finally {
        setLoading(false);
      }
    };

    fetchLeaderboard();
  }, []);

  if (loading) {
    return (
      <Box sx={{ display: 'flex', justifyContent: 'center', mt: 4 }}>
        <CircularProgress />
      </Box>
    );
  }

  if (error) {
    return (
      <Box sx={{ mt: 4 }}>
        <Alert severity="warning">{error}</Alert>
      </Box>
    );
  }

  return (
    <Box sx={{ mt: 4 }}>
      <Typography variant="h4" component="h1" gutterBottom align="center">
        Leaderboard
      </Typography>
      {leaderboard.length === 0 ? (
        <Typography variant="h6" color="textSecondary" align="center">
          Leaderboard data not available yet.
        </Typography>
      ) : (
        <TableContainer component={Paper} sx={{ maxWidth: 600, mx: 'auto' }}>
          <Table>
            <TableHead>
              <TableRow>
                <TableCell>Rank</TableCell>
                <TableCell>Username</TableCell>
                <TableCell align="right">Score</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {leaderboard.map((entry, index) => (
                <TableRow key={entry.address}>
                  <TableCell>{index + 1}</TableCell>
                  <TableCell>{entry.username}</TableCell>
                  <TableCell align="right">{entry.score}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      )}
    </Box>
  );
};

export default Leaderboard;
