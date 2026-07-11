import { composeControllerDeploy, composeControllerRedeploy } from '../../../client/sdk.gen';
import type { ComposeResponseDto } from '../../../client/types.gen';
import { DeploymentsTab } from '../../../components/tabs';

type Props = { compose: ComposeResponseDto };

export default function ComposeDeploymentsTab(props: Props) {
  return (
    <DeploymentsTab
      serviceLabel="compose"
      onDeploy={() => composeControllerDeploy({ path: { id: props.compose.id } }).then()}
      onRedeploy={() => composeControllerRedeploy({ path: { id: props.compose.id } }).then()}
    />
  );
}
