export interface DashboardStats {
  tvl: number;
  apy: number;
  totalStrategies: number;
  totalUsers: number;
}

export interface Strategy {
  id: string;
  name: string;
  tvl: number;
  apy: number;
  risk: 'low' | 'medium' | 'high';
  description?: string;
  depositToken: string;
  rewardToken: string;
  dailyAPY?: number;
}

export interface UserStats {
  totalDeposited: number;
  totalEarned: number;
  activeStrategies: number;
} 