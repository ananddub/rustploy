import type { ComposeResponseDto } from '../../../client/types.gen';
import { SchedulesTab } from '../../../components/tabs';

type Props = { compose: ComposeResponseDto };

export default function ComposeSchedulesTab(props: Props) {
  return <SchedulesTab serviceId={props.compose.id} serviceLabel="compose" />;
}
