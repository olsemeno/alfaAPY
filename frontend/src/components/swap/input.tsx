import clsx from "clsx";
import { Input as InputUi, InputProps, Dropdown } from "../ui";

export function Input({
  className,
  tokens,
  token,
  usdValue,
  balance,
  onTokenChange,
  ...props
}: InputProps & {
  token: { icon: string; label: string; value: string };
  tokens: Array<{ icon: string; label: string; value: string }>;
  onTokenChange: (token: string) => unknown;
  usdValue: string;
  balance: string;
}) {
  console.log("rerender")
  return (
    <div className={clsx("relative", className)}>
      <InputUi
        className="px-[5px] pt-[15px] pb-[35px] w-full text-[21px]"
        {...props}
      />
      <div className="absolute right-[20px] top-[20px]">
        {tokens.length && (
          <Dropdown
            values={tokens}
            value={token.value}
            onChange={onTokenChange}
          />
        )}
      </div>
      <div className="absolute bottom-[10px] px-[20px] flex justify-between w-full">
        {token && (
          <>
            <p className="mb-0 text-sm">{usdValue}</p>
            <p className="mb-0 text-sm">
              Balance: {balance} {token.label}
            </p>
          </>
        )}
      </div>
    </div>
  );
}
