import React from 'react';
import Highcharts from 'highcharts';
import HighchartsReact from 'highcharts-react-official';
import colors from 'tailwindcss/colors';

export interface LineChartSeries {
  name: string;
  data: { x: number; y: number }[];
  color: string;
}

interface LineChartProps {
  period: '24h' | '1m' | '1y' | 'all';
  series: LineChartSeries[];
}

// Helper to convert hex to rgba (supports 3 or 6 digit hex)
function hexToRgba(hex: string, alpha: number) {
  let c = hex.replace('#', '');
  if (c.length === 3) {
    c = c[0] + c[0] + c[1] + c[1] + c[2] + c[2];
  }
  if (c.length !== 6) return hex;
  const r = parseInt(c.substring(0, 2), 16);
  const g = parseInt(c.substring(2, 4), 16);
  const b = parseInt(c.substring(4, 6), 16);
  return `rgba(${r},${g},${b},${alpha})`;
}

export const LineChart: React.FC<LineChartProps> = ({ series = [] }) => {
  const chartSeries: Highcharts.SeriesAreaOptions[] = series.map(s => ({
    name: s.name,
    data: s.data,
    type: 'area',
    color: s.color,
    lineWidth: 2,
    marker: { enabled: true, radius: 4, symbol: 'circle', lineWidth: 0 },
    fillColor: {
      linearGradient: { x1: 0, y1: 0, x2: 0, y2: 1 },
      stops: [
        [0, hexToRgba(s.color, 0.3)],
        [1, hexToRgba(s.color, 0)]
      ]
    }
  }));

  const options: Highcharts.Options = {
    title: { text: undefined },
    chart: {
      backgroundColor: 'transparent',
      spacing: [0, 0, 0, 0],
      margin: [40, 0, 60, 0]
    },
    xAxis: {
      type: 'datetime',
      labels: { enabled: false },
      lineWidth: 0,
      gridLineWidth: 0,
      tickWidth: 0
    },
    yAxis: {
      title: { text: undefined },
      labels: { enabled: false },
      gridLineWidth: 0
    },
    series: chartSeries,
    credits: { enabled: false },
    legend: {
      enabled: true,
      align: 'center',
      verticalAlign: 'bottom',
      layout: 'horizontal',
      backgroundColor: '#fff',
      itemStyle: { fontWeight: 'bold', fontSize: '14px', color: colors.gray[700] },
      floating: false,
      y: 10
    },
    tooltip: { enabled: false }
  };

  return (
    <div className="relative w-full h-[300px] pb-8">
      <HighchartsReact highcharts={Highcharts} options={options} />
    </div>
  );
}; 