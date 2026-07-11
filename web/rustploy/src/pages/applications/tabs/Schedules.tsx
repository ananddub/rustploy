import type { ApplicationResponseDto } from '../../../client/types.gen';
import { SchedulesTab } from '../../../components/tabs';

type Props = { app: ApplicationResponseDto };

export default function AppSchedulesTab(props: Props) {
  return <SchedulesTab serviceId={props.app.id} serviceLabel="application" />;
}
