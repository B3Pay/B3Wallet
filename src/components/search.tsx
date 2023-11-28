import { MagnifyingGlassIcon } from "@radix-ui/react-icons"
import { Input } from "components/ui/input"

export function Search() {
  return (
    <Input
      type="search"
      placeholder="Search..."
      className="md:w-[100px] lg:w-[300px]"
      icon={<MagnifyingGlassIcon />}
    />
  )
}
