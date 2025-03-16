import { ConnectWallet } from "./connect-wallet";

export function Header() {
  return (
    <div className="flex py-[20px] md:py-[30px]">
      <div className="ml-auto">
        <ConnectWallet />
      </div>
    </div>
  );
}
