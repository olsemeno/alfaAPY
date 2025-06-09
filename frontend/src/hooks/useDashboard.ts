import { useEffect, useState } from 'react';
import { DashboardStats, Strategy, UserStats } from '../interfaces/dashboard';

export const useDashboard = () => {
  const [loading, setLoading] = useState(true);
  
  const stats: DashboardStats = {
    tvl: 1234567,
    apy: 12.5,
    totalStrategies: 8,
    totalUsers: 1234
  };

  const topStrategies: Strategy[] = [
    {
      id: '1',
      name: 'ICP-USDT LP Strategy',
      tvl: 500000,
      apy: 15.2,
      risk: 'low',
      description: 'Earn rewards by providing liquidity to ICP-USDT pair',
      depositToken: 'ICP-USDT LP',
      rewardToken: 'ICP',
      dailyAPY: 0.041
    },
    {
      id: '2',
      name: 'ICP Staking',
      tvl: 3000,
      apy: 18.5,
      risk: 'low',
      description: 'Stake ICP to earn native rewards',
      depositToken: 'ICP',
      rewardToken: 'ICP',
      dailyAPY: 0.048
    },
    {
      id: '3',
      name: 'ckBTC-ICP LP',
      tvl: 250000,
      apy: 22.1,
      risk: 'medium',
      description: 'Provide liquidity to ckBTC-ICP pair',
      depositToken: 'ckBTC-ICP LP',
      rewardToken: 'ICP',
      dailyAPY: 0.056
    }
  ];

  const userStats: UserStats = {
    totalDeposited: 1000,
    totalEarned: 125,
    activeStrategies: 2
  };

  useEffect(() => {
    // Simulate API call
    setTimeout(() => setLoading(false), 1000);
  }, []);

  return {
    loading,
    stats,
    topStrategies,
    userStats
  };
}; 