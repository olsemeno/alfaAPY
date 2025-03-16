import clsx from "clsx";
import { Input as PixelInput, InputProps } from "pixel-retroui";
import colors from "tailwindcss/colors";

export type { InputProps } from "pixel-retroui";

export function Input(props: InputProps) {
  return (
    <PixelInput
      {...props}
      className={clsx("m-0", props.className)}
      textColor={colors.black}
    />
  );
}
