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
    <div 
      className="relative inline-flex" 
      style={{ 
        width: logos.length > 1 ? size + ((logos.length - 1) * (size / 1.7)) : size,
        height: size 
      }}
    >
      {logos.map((logo, i) => (
        <div
          key={i}
          className="absolute"
          style={{
            left: `${(size / 1.7) * i}px`,
            width: size,
            height: size,
            zIndex: logos.length - i,
          }}
        >
          <img
          className={clsx(
              "w-full h-full rounded-full border-2 border-black bg-white object-cover",
            className
          )}
            src={logo}
            alt={`Token logo ${i + 1}`}
        />
        </div>
      ))}
    </div>
  );
}
