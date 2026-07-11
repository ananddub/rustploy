import type { ApplicationResponseDto } from '../../../client/types.gen';
import { LogsTab } from '../../../components/tabs';

type Props = { app: ApplicationResponseDto };

export default function AppLogsTab(props: Props) {
  return <LogsTab serviceLabel="application" />;
}
