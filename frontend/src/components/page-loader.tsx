import { motion } from "framer-motion";
import { ProgressBar } from "pixel-retroui";
import { PropsWithChildren, useEffect, useState } from "react";
import colors from "tailwindcss/colors";

export function PageLoader({ children }: PropsWithChildren) {
  const [loadedPercent, setLoadedPercent] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setLoadedPercent((prevProgress) => {
        if (prevProgress >= 100) {
          clearInterval(interval); // Stop if we reach 100%
          return 100;
        }
        return prevProgress + 1; // Increase by 10 every 20ms
      });
    }, 40); // Update progress every 20ms

    // Stop the progress after 2 seconds and show content
    const timeout = setTimeout(() => {
      clearInterval(interval); // Clear the interval
      setLoadedPercent(100);
    }, 4000); // Stop after 2 seconds

    // Cleanup on unmount
    return () => {
      clearInterval(interval);
      clearTimeout(timeout);
    };
  }, []);

  return loadedPercent < 100 ? (
    <div className="h-[100vh] w-[100vw] flex flex-col items-center justify-center">
      <h3>Loading the best APY...</h3>
      <ProgressBar
        size="md"
        color={colors.purple[700]}
        borderColor="black"
        className="w-[300px]"
        progress={loadedPercent}
      />
    </div>
  ) : (
    <>
      <motion.div
        key="3"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        transition={{ duration: 0.3 }}
      >
        {children}
      </motion.div>
    </>
  );
}
