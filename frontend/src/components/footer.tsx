import clsx from "clsx";
import { Replica } from "./ui";

export function Footer({ className }: { className?: string }) {
  return (
    <div
      className={clsx(
        className,
        "flex justify-between py-[20px] md:py-[30px] relative"
      )}
    >
      <span>© 2025 AlphaAPY. All rights reserved.</span>
      <a
        href="https://github.com/olsemeno/alfaAPY/blob/main/README.md"
        target="_blank"
        className="cursor-pointer"
      >
        Docs
      </a>
      <Replica
        className="absolute top-0 right-0 mt-[-35px] ml-[-20px]"
        direction="right"
      >
        True alpha read it first!
      </Replica>
    </div>
  );
}
