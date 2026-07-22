import type { ScheduleMock } from './types.mock';

export const MOCK_SCHEDULES: ScheduleMock[] = [
	{
		id: 'sched-01',
		name: 'Nightly Database Backup',
		cronExpression: '0 2 * * *',
		targetService: 'postgres-db',
		targetProject: 'Rustploy Core Backend',
		status: 'active',
		lastRun: '5h ago (Success)',
		nextRun: 'in 19 hours',
		command: 'pg_dumpall -U postgres | gzip > /backups/db-latest.sql.gz'
	},
	{
		id: 'sched-02',
		name: 'ClickHouse Partition Cleanup',
		cronExpression: '0 0 1 * *',
		targetService: 'analytics-stack',
		targetProject: 'Analytics Data Pipeline',
		status: 'active',
		lastRun: '12 days ago',
		nextRun: 'in 18 days',
		command: 'ALTER TABLE logs DROP PARTITION 202605'
	},
	{
		id: 'sched-03',
		name: 'Redis Cache Warmup',
		cronExpression: '*/30 * * * *',
		targetService: 'api-server',
		targetProject: 'Rustploy Core Backend',
		status: 'paused',
		lastRun: '3 days ago',
		nextRun: 'Paused',
		command: 'curl -X POST http://localhost:8080/api/cache/warmup'
	}
];
