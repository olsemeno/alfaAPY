import clsx from "clsx";
import { Button as PixelButton, ButtonProps } from "pixel-retroui";
import { useState } from "react";
import colors from "tailwindcss/colors";
import CircleLoader from "react-spinners/MoonLoader";

export function Button({
  className,
  loading,
  ...props
}: ButtonProps & { loading?: boolean; disabled?: boolean }) {
  const disabled = loading || props.disabled;
  const [hovered, setHovered] = useState(false);

  const isDisabled = disabled || loading;

  return (
    <PixelButton
      {...props}
      className={clsx(
        "m-0 cursor-pointer transition-all duration-200 ease-in-out flex items-center",
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
      <div className="flex w-full items-center justify-center">
        {loading && (
          <CircleLoader
            className="mr-[5px]"
            color={colors.black}
            loading={true}
            size={15}
          />
        )}
        {props.children}
      </div>
    </PixelButton>
  );
}
