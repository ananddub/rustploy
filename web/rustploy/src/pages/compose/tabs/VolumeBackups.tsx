import type { ComposeResponseDto } from '../../../client/types.gen';
import { VolumeBackupsTab } from '../../../components/tabs';

type Props = { compose: ComposeResponseDto };

export default function ComposeVolumeBackupsTab(props: Props) {
  return <VolumeBackupsTab serviceId={props.compose.id} serviceLabel="compose" />;
}
