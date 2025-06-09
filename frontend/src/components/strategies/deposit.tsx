import { Popup } from "pixel-retroui";
import { Button, Input } from "../ui";
import { useState } from "react";
import BigNumber from "bignumber.js";
import SquareLoader from "react-spinners/ClimbingBoxLoader";
import colors from "tailwindcss/colors";

export function Deposit({
  className,
  isOpen,
  onClose,
  onDeposit,
  onClick,
  balance,
  tokenSymbol,
  loading,
}: {
  className?: string;
  isOpen?: boolean;
  onClose: () => unknown;
  onDeposit: (value: string) => unknown;
  onClick: () => unknown;
  balance: string;
  tokenSymbol: string;
  loading?: boolean;
}) {
  const [value, setValue] = useState("");
  const [inputError, setInputError] = useState("");
  const [showSuccess, setShowSuccess] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);

  const handleDeposit = async (value: string) => {
    try {
      setIsProcessing(true);
      await onDeposit(value);
      setShowSuccess(true);
    } finally {
      setIsProcessing(false);
    }
  };

  if (isProcessing) {
    return (
      <Popup isOpen={!!isOpen} onClose={() => {}} className="modal">
        <div className="flex flex-col items-center justify-center py-12">
          <SquareLoader
            className="mx-auto mb-6"
            color={colors.amber[500]}
            loading={true}
            size={20}
          />
          <h2 className="text-[20px] font-bold mb-2">Processing Deposit</h2>
          <p className="text-gray-600 text-center">
            Please wait while we process your deposit of {value} {tokenSymbol}
          </p>
        </div>
      </Popup>
    );
  }

  return (
    <>
      <Button onClick={onClick} className={className}>
        <span className="text-[20px] block mr-[5px]">📥</span> Deposit
      </Button>
      <Popup
        isOpen={!!isOpen}
        onClose={() => {
          onClose();
          setValue("");
          setInputError("");
          setShowSuccess(false);
        }}
        className="modal"
      >
        {!showSuccess ? (
          <>
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
                disabled={!!inputError || !value}
                loading={loading}
                className="sm:ml-[20px] sm:h-[40px] w-full sm:w-auto sm:h-[42px] mt-[25px] sm:mt-0"
                onClick={() => handleDeposit(value)}
              >
                Confirm
              </Button>
            </div>
          </>
        ) : (
          <div className="text-center py-8">
            <div className="text-6xl mb-6">✅</div>
            <h2 className="text-[24px] font-bold mb-4">Deposit Successful!</h2>
            <p className="text-gray-600 mb-8">
              You have successfully deposited {value} {tokenSymbol}
            </p>
            <Button
              className="w-full sm:w-auto"
              onClick={() => {
                onClose();
                setShowSuccess(false);
                setValue("");
              }}
            >
              Close
            </Button>
          </div>
        )}
      </Popup>
    </>
  );
}
