import { Popup } from "pixel-retroui";
import { Button, Input } from "../ui";
import { useState } from "react";
import BigNumber from "bignumber.js";

export function Deposit({
  className,
  isOpen,
  onClose,
  onDeposit,
  onClick,
  balance,
  tokenSymbol,
  disabled
}: {
  className?: string;
  isOpen?: boolean;
  onClose: () => unknown;
  onDeposit: (value: string) => unknown;
  onClick: () => unknown;
  balance: string;
  tokenSymbol: string;
  disabled?: boolean
}) {
  const [value, setValue] = useState("");
  const [inputError, setInputError] = useState("");
  return (
    <>
      <Button onClick={onClick} className={className}>
        <span className="text-[20px]">📥</span> Deposit
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
          Deposit {tokenSymbol}
        </h2>
        <p className="mb-[35px] sm:mb-[45px]">
          Available balance: {balance} {tokenSymbol}
        </p>
        <div className="flex flex-col sm:flex-row">
          <div className="flex flex-col flex-1">
            <Input
              value={value}
              onChange={(e) => {
                setInputError("");
                const value = e.target.value;
                if (!value) {
                  setInputError("Required");
                } else if (BigNumber(value).gt(balance)) {
                  setInputError("Not enough balance");
                }
                setValue(e.target.value);
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
            disabled={!!inputError || !value || disabled}
            className="sm:ml-[20px] sm:h-[40px] w-full sm:w-auto sm:h-[42px] mt-[25px] sm:mt-0"
            onClick={() => onDeposit(value)}
          >
            Confirm
          </Button>
        </div>
      </Popup>
    </>
  );
}
