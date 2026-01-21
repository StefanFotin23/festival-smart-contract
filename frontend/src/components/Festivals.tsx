import React, { useEffect, useState } from 'react';
import { Box, Typography, CircularProgress, Alert, Card, CardContent, Grid } from '@mui/material';
import { getAllFestivals } from '../services/multiversx.services';

interface Festival {
  id: number;
  name: string;
  startTime: number;
  endTime: number;
  maxTickets: number;
  soldTickets: number;
  insideNow: number;
}

const Festivals = () => {
  const [festivals, setFestivals] = useState<Festival[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchFestivals = async () => {
      try {
        const fetchedFestivals = await getAllFestivals();
        setFestivals(fetchedFestivals);
      } catch (err) {
        console.error('Failed to fetch festivals:', err);
        setError('Failed to load festivals. Please try again later.');
      } finally {
        setLoading(false);
      }
    };

    fetchFestivals();
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
        <Alert severity="error">{error}</Alert>
      </Box>
    );
  }

  return (
    <Box sx={{ mt: 4 }}>
      <Typography variant="h4" component="h1" gutterBottom align="center">
        Upcoming Festivals
      </Typography>
      {festivals.length === 0 ? (
        <Typography variant="h6" color="textSecondary" align="center">
          No festivals available yet.
        </Typography>
      ) : (
        <Grid container spacing={3}>
          {festivals.map((festival) => (
            <Grid item xs={12} sm={6} md={4} key={festival.id}>
              <Card raised sx={{ height: '100%' }}>
                <CardContent>
                  <Typography variant="h5" component="h2" gutterBottom>
                    {festival.name}
                  </Typography>
                  <Typography color="textSecondary">
                    Start: {new Date(festival.startTime * 1000).toLocaleString()}
                  </Typography>
                  <Typography color="textSecondary">
                    End: {new Date(festival.endTime * 1000).toLocaleString()}
                  </Typography>
                  <Typography>Max Tickets: {festival.maxTickets}</Typography>
                  <Typography>Sold Tickets: {festival.soldTickets}</Typography>
                  <Typography>Currently Inside: {festival.insideNow}</Typography>
                  {/* Add more details or a link to a festival detail page */}
                </CardContent>
              </Card>
            </Grid>
          ))}
        </Grid>
      )}
    </Box>
  );
};

export default Festivals;
