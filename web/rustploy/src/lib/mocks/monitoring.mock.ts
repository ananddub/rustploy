export interface TelemetrySeriesPoint {
	timestamp: string;
	cpuCore1: number;
	cpuCore2: number;
	cpuCore3: number;
	cpuCore4: number;
	cpuCore5: number;
	cpuCore6: number;
	cpuCore7: number;
	cpuCore8: number;
	cpuAvg: number;
	ramUsedGb: number;
	ramTotalGb: number;
	ramPercent: number;
	netRxMbps: number;
	netTxMbps: number;
	diskReadIops: number;
	diskWriteIops: number;
	httpRps: number;
	httpLatencyP95Ms: number;
}

export type TelemetryTimeRange = '1m' | '5m' | '15m' | '1d' | '7d' | '1mth' | '1yr' | 'Max';

export function getMonitoringMock(
	hostNode = 'production-01',
	count = 60,
	range: TelemetryTimeRange = '1m'
): TelemetrySeriesPoint[] {
	const now = Date.now();
	const result: TelemetrySeriesPoint[] = [];

	let intervalMs = 1000;
	if (range === '5m') intervalMs = 5000;
	else if (range === '15m') intervalMs = 15000;
	else if (range === '1d') intervalMs = (24 * 60 * 60 * 1000) / count;
	else if (range === '7d') intervalMs = (7 * 24 * 60 * 60 * 1000) / count;
	else if (range === '1mth') intervalMs = (30 * 24 * 60 * 60 * 1000) / count;
	else if (range === '1yr') intervalMs = (365 * 24 * 60 * 60 * 1000) / count;
	else if (range === 'Max') intervalMs = (730 * 24 * 60 * 60 * 1000) / count;

	for (let i = 0; i < count; i++) {
		const targetDate = new Date(now - (count - i) * intervalMs);

		let time = '';
		if (range === '1m' || range === '5m' || range === '15m') {
			time = targetDate.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
		} else if (range === '1d') {
			time = targetDate.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
		} else if (range === '7d' || range === '1mth') {
			time = targetDate.toLocaleDateString([], { month: 'short', day: 'numeric' });
		} else {
			time = targetDate.toLocaleDateString([], { year: '2-digit', month: 'short' });
		}

		const t = i * 0.15;
		const baseLoad = hostNode.includes('staging') ? 35 : hostNode.includes('backup') ? 28 : 52;

		const c1 = Math.min(95, Math.max(10, baseLoad + Math.sin(t) * 22 + Math.random() * 6));
		const c2 = Math.min(95, Math.max(10, baseLoad + Math.cos(t * 1.2) * 18 + Math.random() * 5));
		const c3 = Math.min(95, Math.max(10, baseLoad + Math.sin(t * 0.8) * 25 + Math.random() * 7));
		const c4 = Math.min(95, Math.max(10, baseLoad + Math.cos(t * 1.5) * 20 + Math.random() * 4));
		const c5 = Math.min(95, Math.max(10, baseLoad * 0.85 + Math.sin(t * 1.1) * 16 + Math.random() * 5));
		const c6 = Math.min(95, Math.max(10, baseLoad * 0.9 + Math.cos(t * 0.9) * 21 + Math.random() * 6));
		const c7 = Math.min(95, Math.max(10, baseLoad * 1.05 + Math.sin(t * 1.3) * 24 + Math.random() * 8));
		const c8 = Math.min(95, Math.max(10, baseLoad * 0.8 + Math.cos(t * 0.7) * 14 + Math.random() * 4));

		const cpuAvg = Number(((c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8) / 8).toFixed(1));
		const ramPercent = Number((42 + Math.sin(t * 0.3) * 12 + Math.random() * 2).toFixed(1));
		const ramUsedGb = Number(((ramPercent / 100) * 16).toFixed(2));

		result.push({
			timestamp: time,
			cpuCore1: Number(c1.toFixed(1)),
			cpuCore2: Number(c2.toFixed(1)),
			cpuCore3: Number(c3.toFixed(1)),
			cpuCore4: Number(c4.toFixed(1)),
			cpuCore5: Number(c5.toFixed(1)),
			cpuCore6: Number(c6.toFixed(1)),
			cpuCore7: Number(c7.toFixed(1)),
			cpuCore8: Number(c8.toFixed(1)),
			cpuAvg,
			ramUsedGb,
			ramTotalGb: 16.0,
			ramPercent,
			netRxMbps: Number((48 + Math.sin(t * 0.6) * 22 + Math.random() * 8).toFixed(1)),
			netTxMbps: Number((26 + Math.cos(t * 0.6) * 14 + Math.random() * 4).toFixed(1)),
			diskReadIops: Number((1100 + Math.sin(t * 0.4) * 350 + Math.random() * 150).toFixed(0)),
			diskWriteIops: Number((580 + Math.cos(t * 0.4) * 180 + Math.random() * 80).toFixed(0)),
			httpRps: Number((850 + Math.sin(t * 0.5) * 300 + Math.random() * 100).toFixed(0)),
			httpLatencyP95Ms: Number((14 + Math.sin(t * 0.8) * 6 + Math.random() * 3).toFixed(1))
		});
	}

	return result;
}

export function generateLoopingTick(prev: TelemetrySeriesPoint, tickIndex: number): TelemetrySeriesPoint {
	const timeStr = new Date().toLocaleTimeString([], {
		hour: '2-digit',
		minute: '2-digit',
		second: '2-digit'
	});

	const t = tickIndex * 0.15;
	const c1 = Math.min(95, Math.max(10, 52 + Math.sin(t) * 22 + (Math.random() * 6 - 3)));
	const c2 = Math.min(95, Math.max(10, 48 + Math.cos(t * 1.2) * 18 + (Math.random() * 5 - 2.5)));
	const c3 = Math.min(95, Math.max(10, 56 + Math.sin(t * 0.8) * 25 + (Math.random() * 7 - 3.5)));
	const c4 = Math.min(95, Math.max(10, 45 + Math.cos(t * 1.5) * 20 + (Math.random() * 4 - 2)));
	const c5 = Math.min(95, Math.max(10, 40 + Math.sin(t * 1.1) * 16 + (Math.random() * 5 - 2.5)));
	const c6 = Math.min(95, Math.max(10, 46 + Math.cos(t * 0.9) * 21 + (Math.random() * 6 - 3)));
	const c7 = Math.min(95, Math.max(10, 58 + Math.sin(t * 1.3) * 24 + (Math.random() * 8 - 4)));
	const c8 = Math.min(95, Math.max(10, 38 + Math.cos(t * 0.7) * 14 + (Math.random() * 4 - 2)));

	const cpuAvg = Number(((c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8) / 8).toFixed(1));
	const ramPercent = Number((42 + Math.sin(t * 0.3) * 12 + (Math.random() * 2 - 1)).toFixed(1));
	const ramUsedGb = Number(((ramPercent / 100) * 16).toFixed(2));

	return {
		timestamp: timeStr,
		cpuCore1: Number(c1.toFixed(1)),
		cpuCore2: Number(c2.toFixed(1)),
		cpuCore3: Number(c3.toFixed(1)),
		cpuCore4: Number(c4.toFixed(1)),
		cpuCore5: Number(c5.toFixed(1)),
		cpuCore6: Number(c6.toFixed(1)),
		cpuCore7: Number(c7.toFixed(1)),
		cpuCore8: Number(c8.toFixed(1)),
		cpuAvg,
		ramUsedGb,
		ramTotalGb: 16.0,
		ramPercent,
		netRxMbps: Number((48 + Math.sin(t * 0.6) * 22 + (Math.random() * 8 - 4)).toFixed(1)),
		netTxMbps: Number((26 + Math.cos(t * 0.6) * 14 + (Math.random() * 4 - 2)).toFixed(1)),
		diskReadIops: Number((1100 + Math.sin(t * 0.4) * 350 + (Math.random() * 150 - 75)).toFixed(0)),
		diskWriteIops: Number((580 + Math.cos(t * 0.4) * 180 + (Math.random() * 80 - 40)).toFixed(0)),
		httpRps: Number((850 + Math.sin(t * 0.5) * 300 + (Math.random() * 100 - 50)).toFixed(0)),
		httpLatencyP95Ms: Number((14 + Math.sin(t * 0.8) * 6 + (Math.random() * 3 - 1.5)).toFixed(1))
	};
}
