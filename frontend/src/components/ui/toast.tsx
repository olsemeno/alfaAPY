import clsx from "clsx";
import { Id, toast, ToastOptions } from "react-toastify";

// eslint-disable-next-line react-refresh/only-export-components
const Toast = ({ text }: { title?: string; text?: string }) => {
  return (
    <div className={clsx("text-sm text-black")}>
      {text ? <p className="font-normal">{text}</p> : null}
    </div>
  );
};

const toaster = (
  myProps: { title?: string; text?: string },
  toastProps?: ToastOptions
) => toast(<Toast {...myProps} />, { ...toastProps });

toaster.success = (text?: string, toastProps?: ToastOptions): Id =>
  toast.success(<Toast title="Success notification" text={text} />, {
    icon: <>✅</>,
    closeOnClick: true,
    autoClose: false,
    ...toastProps,
  });

toaster.error = (text?: string, toastProps?: ToastOptions): Id =>
  toast.error(<Toast title="Error notification" text={text} />, {
    icon: <>❌</>,
    closeOnClick: true,
    ...toastProps,
  });

export default toaster;
