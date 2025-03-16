import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "pixel-retroui";
import colors from "tailwindcss/colors";

function Item({ icon, label }: { icon: string; label: string }) {
  return (
    <div className="flex items-center">
      <img className="rounded-[50%] mr-[8px] h-[25px] w-[25px]" src={icon} />
      {label}
    </div>
  );
}

export function Dropdown({
  values,
  value,
  onChange,
}: {
  values: Array<{ icon: string; label: string; value: string }>;
  value: string;
  onChange: (value: string) => unknown;
}) {
  const currentItem = values.find((v) => v.value === value);
  return (
    <DropdownMenu>
      <DropdownMenuTrigger bg={colors.amber[50]}>
        <Item icon={currentItem!.icon} label={currentItem!.label} />
      </DropdownMenuTrigger>
      <DropdownMenuContent bg={colors.amber[50]} className="max-h-[200px] overflow-y-scroll">
        {values.map(({ value, label, icon }) => (
          <DropdownMenuItem className="px-[5px] py-[2.5px] hover:bg-amber-100" key={value}>
            <div onClick={() => onChange(value)}>
              <Item icon={icon} label={label} />
            </div>
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
