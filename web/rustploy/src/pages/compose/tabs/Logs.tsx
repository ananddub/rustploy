import type { ComposeResponseDto } from '../../../client/types.gen';
import { LogsTab } from '../../../components/tabs';

type Props = { compose: ComposeResponseDto };

export default function ComposeLogsTab(props: Props) {
  return <LogsTab serviceLabel="compose" />;
}
