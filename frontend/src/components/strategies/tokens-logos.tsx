import clsx from "clsx";

export function TokensLogos({
  logos,
  size = 40,
  className,
}: {
  logos: string[];
  size?: number;
  className?: string;
}) {
  return (
    <>
      {logos.map((l, i) => (
        <img
          style={{
            marginLeft: `${(size / 2) * i}px`,
            marginTop: `${i ? -size : "0"}px`,
            zIndex: i + 1,
            width: size,
            height: size,
          }}
          className={clsx(
            `relative rounded-[50%] border-2 border-black`,
            className
          )}
          key={i}
          src={l}
        />
      ))}
    </>
  );
}
