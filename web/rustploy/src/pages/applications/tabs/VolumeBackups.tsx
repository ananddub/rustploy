import type { ApplicationResponseDto } from '../../../client/types.gen';
import { VolumeBackupsTab } from '../../../components/tabs';

type Props = { app: ApplicationResponseDto };

export default function AppVolumeBackupsTab(props: Props) {
  return <VolumeBackupsTab serviceId={props.app.id} serviceLabel="application" />;
}
