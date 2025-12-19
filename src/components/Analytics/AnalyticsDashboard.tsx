import React, { useEffect, useState } from 'react';
import {
  LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer,
  PieChart, Pie, Cell, Legend
} from 'recharts';
import { analyticsService, WatchStats, DailyWatchStat, MediaTypeStat } from '../../services/analyticsService';
import './AnalyticsDashboard.css';

const COLORS = ['#e50914', '#00C49F', '#FFBB28', '#FF8042', '#8884d8'];

export const AnalyticsDashboard: React.FC = () => {
  const [stats, setStats] = useState<WatchStats | null>(null);
  const [history, setHistory] = useState<DailyWatchStat[]>([]);
  const [distribution, setDistribution] = useState<MediaTypeStat[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setLoading(true);
    try {
      const [statsData, historyData, distData] = await Promise.all([
        analyticsService.getWatchStats(),
        analyticsService.getWatchHistoryChart(30),
        analyticsService.getMediaTypeDistribution()
      ]);
      setStats(statsData);
      setHistory(historyData);
      setDistribution(distData);
    } catch (err) {
      console.error('Failed to load analytics', err);
    } finally {
      setLoading(false);
    }
  };

  const formatDuration = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    return `${hours}h`;
  };

  if (loading) return <div className="analytics-loading">Loading analytics...</div>;

  return (
    <div className="analytics-dashboard">
      <h2 className="analytics-title">Library Analytics</h2>

      {/* Summary Cards */}
      <div className="analytics-grid">
        <div className="analytics-card">
          <h3>Total Watched</h3>
          <p className="analytics-value">{stats?.total_watched}</p>
        </div>
        <div className="analytics-card">
          <h3>In Progress</h3>
          <p className="analytics-value">{stats?.total_in_progress}</p>
        </div>
        <div className="analytics-card">
          <h3>Total Watch Time</h3>
          <p className="analytics-value">{formatDuration(stats?.total_watch_time || 0)}</p>
        </div>
        <div className="analytics-card">
          <h3>Total Sessions</h3>
          <p className="analytics-value">{stats?.total_sessions}</p>
        </div>
      </div>

      <div className="analytics-charts-row">
        {/* Watch History Line Chart */}
        <div className="analytics-chart-container">
          <h3>Activity (Last 30 Days)</h3>
          <div style={{ width: '100%', height: 300 }}>
            <ResponsiveContainer>
              <LineChart data={history}>
                <CartesianGrid strokeDasharray="3 3" stroke="#333" />
                <XAxis dataKey="date" stroke="#888" />
                <YAxis stroke="#888" />
                <Tooltip
                  contentStyle={{ backgroundColor: '#1a1a1a', border: '1px solid #333' }}
                  labelStyle={{ color: '#fff' }}
                />
                <Line type="monotone" dataKey="minutes" stroke="#e50914" strokeWidth={2} dot={false} />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Media Distribution Pie Chart */}
        <div className="analytics-chart-container">
          <h3>Library Distribution</h3>
          <div style={{ width: '100%', height: 300 }}>
            <ResponsiveContainer>
              <PieChart>
                <Pie
                  data={distribution as any}
                  cx="50%"
                  cy="50%"
                  innerRadius={60}
                  outerRadius={80}
                  fill="#8884d8"
                  paddingAngle={5}
                  dataKey="count"
                  nameKey="media_type"
                >
                  {distribution.map((_, index) => (
                    <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                  ))}
                </Pie>
                <Tooltip
                  contentStyle={{ backgroundColor: '#1a1a1a', border: '1px solid #333' }}
                  itemStyle={{ color: '#fff' }}
                />
                <Legend />
              </PieChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>
    </div>
  );
};
