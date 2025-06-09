import { Popup } from "pixel-retroui";
import { Button, Input } from "../ui";
import { useState } from "react";
import SquareLoader from "react-spinners/ClimbingBoxLoader";
import colors from "tailwindcss/colors";

export function Withdraw({
  className,
  isOpen,
  onClose,
  onWithdraw,
  onClick,
  available,
  tokenSymbol,
  loading,
}: {
  className?: string;
  isOpen?: boolean;
  onClose: () => unknown;
  onWithdraw: (value: number) => unknown;
  onClick: () => unknown;
  available: string;
  tokenSymbol: string;
  loading?: boolean;
}) {
  const [value, setValue] = useState("");
  const [inputError, setInputError] = useState("");
  const [showSuccess, setShowSuccess] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);

  const handleWithdraw = async (value: string) => {
    try {
      setIsProcessing(true);
      await onWithdraw(Number(value));
      setShowSuccess(true);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleSliderChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = e.target.value;
    setValue(newValue);
    setInputError("");
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setInputError("");
    const newValue = e.target.value.replace('%', '');
    if (!newValue) {
      setInputError("Required");
    } else if (Number(newValue) > 100) {
      setInputError("Cannot withdraw more than 100%");
    } else if (Number(newValue) <= 0) {
      setInputError("Must be greater than 0");
    }
    setValue(newValue);
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
          <h2 className="text-[20px] font-bold mb-2">Processing Withdrawal</h2>
          <p className="text-gray-600 text-center">
            Please wait while we process your withdrawal of {value}% of your position
          </p>
        </div>
      </Popup>
    );
  }

  return (
    <>
      <Button onClick={onClick} className={className}>
        <span className="text-[20px] block mr-[5px]">📤</span> Withdraw
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
              Withdraw {tokenSymbol}
            </h2>
            <p className="mb-[35px] sm:mb-[45px]">
              Available to withdraw: {available} {tokenSymbol}
            </p>
            <div className="flex flex-col gap-6">
              <div className="flex flex-col gap-2">
                <input
                  type="range"
                  min="0"
                  max="100"
                  value={value || "0"}
                  onChange={handleSliderChange}
                  className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                />
                <div className="flex justify-between text-xs text-gray-500">
                  <span>0%</span>
                  <span>25%</span>
                  <span>50%</span>
                  <span>75%</span>
                  <span>100%</span>
                </div>
              </div>
              <div className="flex flex-col sm:flex-row">
                <div className="flex flex-col flex-1">
                  <div className="relative">
                    <Input
                      value={value ? `${value}%` : ""}
                      onChange={handleInputChange}
                      className="flex-1 pr-8"
                      type="text"
                      placeholder="Enter percentage"
                    />
                  </div>
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
                  onClick={() => handleWithdraw(value)}
                >
                  Confirm
                </Button>
              </div>
            </div>
          </>
        ) : (
          <div className="text-center py-8">
            <div className="text-6xl mb-6">✅</div>
            <h2 className="text-[24px] font-bold mb-4">Withdrawal Successful!</h2>
            <p className="text-gray-600 mb-8">
              You have successfully withdrawn {value}% of your position
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
