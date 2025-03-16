import wolfMoonImg from "../../assets/wolf-moon.png";

export function Lending() {
  return (
    <div className="flex flex-col items-center my-auto">
      <h2 className="relative z-[1]">Howling will not take you to the moon, lending - will!</h2>
      <img src={wolfMoonImg} className="h-[300px] mr-[-390px] mt-[-60px]" />
    </div>
  );
}
