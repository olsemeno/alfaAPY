/* eslint-disable @typescript-eslint/no-explicit-any */
import {
  ConnectWalletDropdownMenu,
  ConnectWalletDropdownMenuAddressItem,
  ConnectWalletDropdownMenuButton,
  ConnectWalletDropdownMenuDisconnectItem,
  ConnectWalletDropdownMenuItems,
} from "@nfid/identitykit/react";
import { ConnectedButton } from "./connected-button";
import { ConnectButton } from "./connect-button";
import { Card } from "../ui";

export function DropdownMenu({
  connectedAccount,
  icpBalance,
  disconnect,
}: any) {
  return (
    <ConnectWalletDropdownMenu className="p-0">
      <ConnectWalletDropdownMenuButton>
        {connectedAccount ? (
          <ConnectedButton
            connectedAccount={connectedAccount}
            icpBalance={icpBalance}
          />
        ) : (
          <ConnectButton />
        )}
      </ConnectWalletDropdownMenuButton>
      <ConnectWalletDropdownMenuItems className="connect-wallet-dropdown !p-0 !rounded-0 !shadow-none !bg-transparent !min-w-[405px] sm:!min-w-[420px]">
        <Card className="py-[5px] px-[10px] text-center connected-wallet-dropdown">
          <ConnectWalletDropdownMenuDisconnectItem
            className="!px-0 !py-[5px]"
            onClick={disconnect}
          />
          {/* <ConnectWalletDropdownMenuItem className="!px-0 !py-[5px]">
            <div className="flex w-full justify-between font-bold">
              <h2>Total balance: $0.00</h2>
              <h2 className="cursor-pointer">
                <span className="!text-[25px] mr-[4px]">🔄</span>Swap
              </h2>
            </div>
          </ConnectWalletDropdownMenuItem> */}
          <ConnectWalletDropdownMenuAddressItem
            value={connectedAccount}
            className="!px-0 !py-[5px]"
          />
        </Card>
      </ConnectWalletDropdownMenuItems>
    </ConnectWalletDropdownMenu>
  );
}
