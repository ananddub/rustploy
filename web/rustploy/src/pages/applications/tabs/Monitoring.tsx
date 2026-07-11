import type { ApplicationResponseDto } from '../../../client/types.gen';
import { MonitoringTab } from '../../../components/tabs';

type Props = { app: ApplicationResponseDto };

export default function AppMonitoringTab(props: Props) {
  return <MonitoringTab serviceLabel="application" />;
}
