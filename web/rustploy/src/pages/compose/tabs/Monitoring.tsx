import type { ComposeResponseDto } from '../../../client/types.gen';
import { MonitoringTab } from '../../../components/tabs';

type Props = { compose: ComposeResponseDto };

export default function ComposeMonitoringTab(props: Props) {
  return <MonitoringTab serviceLabel="compose" />;
}
