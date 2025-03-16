import clsx from "clsx";
import { Button as PixelButton, ButtonProps } from "pixel-retroui";
import { useState } from "react";
import colors from "tailwindcss/colors";

export function Button({
  className,
  disabled,
  loading,
  ...props
}: ButtonProps & { loading?: boolean; disabled?: boolean }) {
  const [hovered, setHovered] = useState(false);

  const isDisabled = disabled || loading;

  return (
    <PixelButton
      {...props}
      className={clsx(
        "m-0 cursor-pointer",
        { "cursor-disabled": isDisabled },
        className
      )}
      bg={
        isDisabled
          ? colors.gray[300]
          : hovered
          ? colors.amber[500]
          : props.bg ?? colors.amber[400]
      }
      shadow={isDisabled ? colors.gray[500] : colors.amber[hovered ? 700 : 600]}
      textColor={colors.black}
      onMouseOver={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      {loading ? "Loading..." : props.children}
    </PixelButton>
  );
}
