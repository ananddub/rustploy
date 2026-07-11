import { applicationControllerDeploy, applicationControllerRedeploy } from '../../../client/sdk.gen';
import type { ApplicationResponseDto } from '../../../client/types.gen';
import { DeploymentsTab } from '../../../components/tabs';

type Props = { app: ApplicationResponseDto };

export default function AppDeploymentsTab(props: Props) {
  return (
    <DeploymentsTab
      serviceLabel="application"
      onDeploy={() => applicationControllerDeploy({ path: { id: props.app.id } }).then()}
      onRedeploy={() => applicationControllerRedeploy({ path: { id: props.app.id } }).then()}
    />
  );
}
