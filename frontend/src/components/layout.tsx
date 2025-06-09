import { PropsWithChildren } from "react";
import { Header } from "./header";
import { Footer } from "./footer";
import { PageLoader } from "./page-loader";
import { Badge } from "./ui";
import { useLocation, useNavigate } from "react-router-dom";

export function Layout({ children }: PropsWithChildren) {
  const navigate = useNavigate();
  const location = useLocation();
  return (
    <PageLoader>
      <div className="container w-full max-w-[1341px] px-[10px] md:px-[20px] lg:px-[30px] mx-auto min-h-[100vh] flex flex-col">
        <Header />
        <h1 className="text-center text-[36px]">
          Dominate DeFi with highest APY
        </h1>
        <Badge className="mx-auto !mt-[10px] mb-[70px]">Alpha version</Badge>
        {location.pathname === "/swap" && (
          <div className="w-full mb-[20px]">
            <button
              onClick={() => navigate("/")}
              className="text-gray-600 hover:text-gray-800 transition-colors text-[20px] ml-[20px]"
            >
              ← Back
            </button>
          </div>
        )}
        <div className="my-auto pb-[50px]">{children}</div>
        <Footer className="mt-auto" />
      </div>
    </PageLoader>
  );
}
