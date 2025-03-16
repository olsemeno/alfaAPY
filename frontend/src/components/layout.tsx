import { PropsWithChildren } from "react";
import { Header } from "./header";
import { Footer } from "./footer";
import { PageLoader } from "./page-loader";
import { Badge, Button, Card } from "./ui";
import { useLocation, useNavigate } from "react-router-dom";
import colors from "tailwindcss/colors";

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
          <Card className="w-fit mb-[30px] mx-auto grid grid-cols-3 gap-5 p-[5px]">
            <Button bg={colors.amber[500]} className="text-[16px]">
              <span className="text-[20px]">🔄</span> Swap
            </Button>
            <Button
              onClick={() => {
                navigate("/");
              }}
              bg={colors.amber[400]}
              className="text-[16px]"
            >
              <span className="text-[20px]">🏊</span> Pools
            </Button>
            <Button
              bg={colors.amber[400]}
              className="text-[16px]"
              onClick={() => {
                navigate("/lending");
              }}
            >
              <span className="text-[20px]">💸</span> Lending
            </Button>
          </Card>
        )}
        <div className="my-auto pb-[50px]">{children}</div>
        <Footer className="mt-auto" />
      </div>
    </PageLoader>
  );
}
