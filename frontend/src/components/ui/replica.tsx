import clsx from "clsx";
import { Bubble, BubbleProps } from "pixel-retroui";
import colors from "tailwindcss/colors";

export function Replica(props: BubbleProps) {
  return (
    <Bubble
      bg={colors.purple[200]}
      textColor="black"
      borderColor="black"
      {...props}
      className={clsx("text-xs p-[10px]", props.className)}
    >
      {props.children}
    </Bubble>
  );
}
