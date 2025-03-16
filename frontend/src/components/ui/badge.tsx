import clsx from "clsx";
import { Card as PixelCard } from "pixel-retroui";
import colors from "tailwindcss/colors";

export function Badge({
  children,
  className,
}: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <PixelCard
      className={clsx(
        className,
        "h-[20px] text-[12px] p-0 leading-[10px] my-0 w-fit"
      )}
      bg={colors.purple[400]}
      shadowColor={colors.purple[600]}
      textColor={colors.black}
    >
      {children}
    </PixelCard>
  );
}
