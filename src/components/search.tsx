import { MagnifyingGlassIcon } from "@radix-ui/react-icons"
import { Input } from "components/ui/input"
import { cn } from "lib/utils"

interface searchProps {
  className?: string
}

const search: React.FC<searchProps> = ({ className }) => {
  return (
    <Input
      type="search"
      placeholder="Search..."
      className={cn("md:w-[100px] lg:w-[300px]", className)}
      icon={<MagnifyingGlassIcon />}
    />
  )
}

export default search
