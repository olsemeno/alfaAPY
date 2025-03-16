import clsx from "clsx";
import { Card as PixelCard, CardProps } from "pixel-retroui";
import colors from "tailwindcss/colors";

export function Card({ children, className, ...props }: CardProps) {
  return (
    <PixelCard
      className={clsx("m-0", className)}
      bg={colors.amber[200]}
      shadowColor={colors.amber[600]}
      textColor={colors.black}
      {...props}
    >
      {children}
    </PixelCard>
  );
}
