import clsx from "clsx";
import { Card as PixelCard, CardProps } from "pixel-retroui";
import colors from "tailwindcss/colors";

export function Card({
  children,
  className,
  light,
  ...props
}: CardProps & { light?: boolean }) {
  return (
    <PixelCard
      className={clsx("m-0", className)}
      bg={light ? colors.amber[100] : colors.amber[200]}
      shadowColor={colors.amber[600]}
      textColor={colors.black}
      {...props}
    >
      {children}
    </PixelCard>
  );
}
