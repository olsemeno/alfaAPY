import BlockiesSvg from "blockies-react-svg";

export function Avatar({
  address,
  className,
}: {
  address: string;
  className?: string;
}) {
  return <BlockiesSvg size={3} address={address} className={className} />;
}
