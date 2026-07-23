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

export function getMonitoringMock(hostNode = 'production-01', count = 60): TelemetrySeriesPoint[] {
	const now = Date.now();
	const result: TelemetrySeriesPoint[] = [];

	for (let i = 0; i < count; i++) {
		const time = new Date(now - (count - i) * 1000).toLocaleTimeString([], {
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit'
		});

		const t = i * 0.15;
		const baseLoad = hostNode.includes('staging') ? 12 : hostNode.includes('backup') ? 8 : 22;

		const c1 = Math.min(100, Math.max(4, baseLoad + Math.sin(t) * 18 + Math.random() * 6));
		const c2 = Math.min(100, Math.max(4, baseLoad + Math.cos(t * 1.2) * 14 + Math.random() * 5));
		const c3 = Math.min(100, Math.max(4, baseLoad + Math.sin(t * 0.8) * 22 + Math.random() * 7));
		const c4 = Math.min(100, Math.max(4, baseLoad + Math.cos(t * 1.5) * 16 + Math.random() * 4));
		const c5 = Math.min(100, Math.max(4, baseLoad * 0.8 + Math.sin(t * 1.1) * 12 + Math.random() * 5));
		const c6 = Math.min(100, Math.max(4, baseLoad * 0.9 + Math.cos(t * 0.9) * 15 + Math.random() * 6));
		const c7 = Math.min(100, Math.max(4, baseLoad * 1.1 + Math.sin(t * 1.3) * 20 + Math.random() * 8));
		const c8 = Math.min(100, Math.max(4, baseLoad * 0.75 + Math.cos(t * 0.7) * 10 + Math.random() * 4));

		const cpuAvg = Number(((c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8) / 8).toFixed(1));
		const ramPercent = Number((32 + Math.sin(t * 0.3) * 6 + Math.random() * 2).toFixed(1));
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
			netRxMbps: Number((45 + Math.sin(t * 0.6) * 25 + Math.random() * 8).toFixed(1)),
			netTxMbps: Number((22 + Math.cos(t * 0.6) * 12 + Math.random() * 4).toFixed(1)),
			diskReadIops: Number((1200 + Math.sin(t * 0.4) * 400 + Math.random() * 150).toFixed(0)),
			diskWriteIops: Number((650 + Math.cos(t * 0.4) * 200 + Math.random() * 80).toFixed(0)),
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
	const c1 = Math.min(100, Math.max(4, 20 + Math.sin(t) * 18 + (Math.random() * 6 - 3)));
	const c2 = Math.min(100, Math.max(4, 25 + Math.cos(t * 1.2) * 14 + (Math.random() * 5 - 2.5)));
	const c3 = Math.min(100, Math.max(4, 18 + Math.sin(t * 0.8) * 22 + (Math.random() * 7 - 3.5)));
	const c4 = Math.min(100, Math.max(4, 28 + Math.cos(t * 1.5) * 16 + (Math.random() * 4 - 2)));
	const c5 = Math.min(100, Math.max(4, 16 + Math.sin(t * 1.1) * 12 + (Math.random() * 5 - 2.5)));
	const c6 = Math.min(100, Math.max(4, 22 + Math.cos(t * 0.9) * 15 + (Math.random() * 6 - 3)));
	const c7 = Math.min(100, Math.max(4, 30 + Math.sin(t * 1.3) * 20 + (Math.random() * 8 - 4)));
	const c8 = Math.min(100, Math.max(4, 15 + Math.cos(t * 0.7) * 10 + (Math.random() * 4 - 2)));

	const cpuAvg = Number(((c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8) / 8).toFixed(1));
	const ramPercent = Number((32 + Math.sin(t * 0.3) * 6 + (Math.random() * 2 - 1)).toFixed(1));
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
		netRxMbps: Number((45 + Math.sin(t * 0.6) * 25 + (Math.random() * 8 - 4)).toFixed(1)),
		netTxMbps: Number((22 + Math.cos(t * 0.6) * 12 + (Math.random() * 4 - 2)).toFixed(1)),
		diskReadIops: Number((1200 + Math.sin(t * 0.4) * 400 + (Math.random() * 150 - 75)).toFixed(0)),
		diskWriteIops: Number((650 + Math.cos(t * 0.4) * 200 + (Math.random() * 80 - 40)).toFixed(0)),
		httpRps: Number((850 + Math.sin(t * 0.5) * 300 + (Math.random() * 100 - 50)).toFixed(0)),
		httpLatencyP95Ms: Number((14 + Math.sin(t * 0.8) * 6 + (Math.random() * 3 - 1.5)).toFixed(1))
	};
}
