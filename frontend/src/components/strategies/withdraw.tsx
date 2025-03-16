import { Popup } from "pixel-retroui";
import { Button, Input } from "../ui";
import { useState } from "react";
import BigNumber from "bignumber.js";

export function Withdraw({
  className,
  isOpen,
  onClose,
  onWithdraw,
  onClick,
  available,
  tokenSymbol,
  disabled,
}: {
  className?: string;
  isOpen?: boolean;
  onClose: () => unknown;
  onWithdraw: (percent: string) => unknown;
  onClick: () => unknown;
  available: string;
  tokenSymbol: string;
  disabled?: boolean;
}) {
  const [inputFocused, setInputFocused] = useState(false);
  const [inputError, setInputError] = useState("");
  const [value, setValue] = useState("");
  const isZeroAvailbale = BigNumber(available).eq(0);

  const percent = isZeroAvailbale
    ? "0"
    : BigNumber(value || 0)
        .div(available)
        .multipliedBy(100)
        .toString();

  return (
    <>
      <Button onClick={onClick} className={className}>
        <span className="text-[20px]">📤</span> Withdraw
      </Button>
      <Popup
        isOpen={!!isOpen}
        onClose={() => {
          onClose();
          setValue("");
          setInputError("");
        }}
        className="modal"
      >
        <h2 className="text-[18px] sm:text-[20px] mb-[10px]">
          Withdraw {tokenSymbol}
        </h2>
        <p className="mb-[35px] sm:mb-[45px]">
          Available balance: {available} {tokenSymbol}
        </p>
        <input
          disabled={isZeroAvailbale}
          id="small-range"
          type="range"
          value={percent}
          onChange={(e) => {
            setValue(
              BigNumber(e.target.value || 0)
                .div(100)
                .multipliedBy(available)
                .toString()
            );
          }}
          className="w-full h-1 bg-gray-200 rounded-lg appearance-none cursor-pointer range-sm dark:bg-gray-700"
        />
        <div className="mb-6 flex justify-between">
          {[0, 25, 50, 75, 100].map((el) => (
            <span className="text-xs">{el}%</span>
          ))}
        </div>
        <div className="flex flex-col sm:flex-row">
          <div className="flex flex-col flex-1">
            <Input
              onFocus={() => setInputFocused(true)}
              onBlur={() => setInputFocused(false)}
              value={value}
              onChange={(e) => {
                if (inputFocused) {
                  setInputError("");
                  const value = e.target.value;
                  if (!value) {
                    setInputError("Required");
                  } else if (BigNumber(value).gt(available)) {
                    setInputError("Not enough balance");
                  }
                  setValue(e.target.value);
                }
              }}
              className="flex-1"
              type="number"
              min={0}
            />
            {inputError && (
              <span className="h-4 mt-[10px] text-xs leading-4 text-red-600">
                {inputError}
              </span>
            )}
          </div>
          <Button
            disabled={isZeroAvailbale || disabled || !value}
            className="sm:ml-[20px] sm:h-[40px] w-full sm:w-auto sm:h-[42px] mt-[25px] sm:mt-0"
            onClick={() => onWithdraw(percent)}
          >
            Confirm
          </Button>
        </div>
      </Popup>
    </>
  );
}
