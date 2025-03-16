/* eslint-disable @typescript-eslint/no-explicit-any */
import { Avatar } from "./avatar";
import { Button } from "../ui";

export function ConnectedButton({
  connectedAccount,
  icpBalance,
  ...props
}: any) {
  return (
    <Button {...props}>
      <div className="flex items-center">
        <Avatar className="mr-[8px]" address={connectedAccount} />
        <span className="text-xs">{`${connectedAccount.substring(
          0,
          5
        )}...${connectedAccount.substring(connectedAccount.length - 5)}`}</span>
      </div>
    </Button>
  );
}
