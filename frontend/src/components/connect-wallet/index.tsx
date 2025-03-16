import { ConnectWallet as IkConnectWallet } from "@nfid/identitykit/react";
import { ConnectButton } from "./connect-button";
import { ConnectedButton } from "./connected-button";
import { DropdownMenu } from "./dropdown";

export function ConnectWallet() {
  return (
    <IkConnectWallet
      connectButtonComponent={ConnectButton}
      connectedButtonComponent={ConnectedButton}
      dropdownMenuComponent={DropdownMenu}
    />
  );
}
