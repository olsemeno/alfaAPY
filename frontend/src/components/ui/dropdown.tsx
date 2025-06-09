import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "pixel-retroui";
import colors from "tailwindcss/colors";
import { useState, useMemo } from "react";
import { Input } from "./input";

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
  const [searchQuery, setSearchQuery] = useState("");
  const currentItem = values.find((v) => v.value === value);

  const filteredValues = useMemo(() => {
    if (!searchQuery) return values;
    const query = searchQuery.toLowerCase();
    return values.filter(
      (item) => item.label.toLowerCase().includes(query)
    );
  }, [values, searchQuery]);

  return (
    <DropdownMenu>
      <DropdownMenuTrigger bg={colors.amber[50]}>
        <Item icon={currentItem!.icon} label={currentItem!.label} />
      </DropdownMenuTrigger>
      <DropdownMenuContent bg={colors.amber[50]} className="max-h-[300px] overflow-y-scroll">
        <div className="p-2">
          <Input
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search tokens..."
            className="w-full"
          />
        </div>
        {filteredValues.map(({ value, label, icon }) => (
          <DropdownMenuItem className="px-[5px] py-[2.5px] hover:bg-amber-100" key={value}>
            <div onClick={() => onChange(value)}>
              <Item icon={icon} label={label} />
            </div>
          </DropdownMenuItem>
        ))}
        {filteredValues.length === 0 && (
          <div className="px-4 py-2 text-sm text-gray-500">
            No tokens found
          </div>
        )}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
