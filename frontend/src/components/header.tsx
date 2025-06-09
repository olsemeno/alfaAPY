import { ConnectWallet } from "./connect-wallet";
import logo from "../assets/logo.png";
import { useNavigate } from "react-router-dom";

export function Header() {
  const navigate = useNavigate();
  return (
    <div className="flex py-[20px] md:py-[30px] justify-between items-center">
      <img
        onClick={() => {
          navigate("/");
        }}
        src={logo}
        alt="logo"
        className="w-[100px] cursor-pointer"
      />
      <div className="flex items-center gap-4">
        <div
          onClick={() => navigate("/swap")}
          className="flex items-center justify-between cursor-pointer gap-4"
        >
          <h2 className="mb-0 font-bold">$0.00</h2>
          <h2 className="mb-0 flex items-center">
            <span className="!text-[18px] mr-[4px]">🔄</span>Swap
          </h2>
        </div>
        <div
          onClick={() => navigate("/profile")}
          className="flex items-center justify-between cursor-pointer"
        >
          <h2 className="mb-0">
            <span className="!text-[18px] mr-[4px]">👤</span>Profile
          </h2>
        </div>
        <ConnectWallet />
      </div>
    </div>
  );
}
