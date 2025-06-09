import clsx from "clsx";
import { Replica } from "./ui";

export function Footer({ className }: { className?: string }) {
  return (
    <a
      href="https://github.com/olsemeno/alfaAPY/blob/main/README.md"
      target="_blank"
      className={clsx(
        className,
        "flex justify-between py-[20px] md:py-[30px] relative cursor-pointer hover:opacity-80 transition-opacity"
      )}
    >
      <span>© 2025 AlphaAPY. All rights reserved.</span>
      <span>Docs</span>
      <Replica
        className="absolute top-0 right-0 mt-[-35px] ml-[-20px]"
        direction="right"
      >
        True alpha reads it first!
      </Replica>
    </a>
  );
}
