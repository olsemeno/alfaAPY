import { ConnectWalletButtonProps } from "@nfid/identitykit/react";
import { Button } from "../ui";

export function ConnectButton(props: ConnectWalletButtonProps) {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return <Button {...props as any}>Connect wallet</Button>;
}
