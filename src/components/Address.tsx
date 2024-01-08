import { CheckIcon, CopyIcon } from "@radix-ui/react-icons"
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger
} from "@src/components/ui/tooltip"
import { cn } from "@src/lib/utils"
import { useState } from "react"
import { Button } from "./ui/button"
import { DropdownMenuShortcut } from "./ui/dropdown-menu"
import useTruncated from "@src/lib/useTruncated"

interface AddressWithCopyProps
  extends React.HTMLAttributes<HTMLParagraphElement> {
  address: string
  noIcon?: boolean
  hiddenAddress?: boolean
  iconSize?: "xs" | "sm" | "md" | "lg" | "xl" | null
  size?: "xs" | "sm" | "md" | "lg" | "xl" | null
  asMenuItem?: boolean
  prefix?: string
}

const Address: React.FC<AddressWithCopyProps> = ({
  address,
  noIcon,
  hiddenAddress,
  className,
  asMenuItem,
  prefix,
  size = "md",
  iconSize = "md",
  ...rest
}) => {
  const IconComp = asMenuItem ? DropdownMenuShortcut : "span"

  const [hasCopied, setHasCopied] = useState(false)

  const onCopy = () => {
    navigator.clipboard.writeText(address)
    setHasCopied(true)
    setTimeout(() => setHasCopied(false), 2000)
  }

  const truncatedAddress = useTruncated(address, size)

  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="flex items-center overflow-hidden" title={address}>
            <span
              className={`overflow-hidden overflow-ellipsis whitespace-nowrap ${
                hiddenAddress ? "hidden" : ""
              }`}
            >
              {prefix && <span className="mr-1">{prefix}</span>}
              {truncatedAddress}
            </span>
            {!noIcon && (
              <IconComp>
                <Button
                  variant="link"
                  size={iconSize}
                  bgColor="inherit"
                  onClick={onCopy}
                  asIconButton
                  aria-label="Copy to clipboard"
                >
                  {hasCopied ? (
                    <CheckIcon className="h-4 w-4" />
                  ) : (
                    <CopyIcon className="h-4 w-4" />
                  )}
                </Button>
              </IconComp>
            )}
          </span>
        </TooltipTrigger>
        <TooltipContent>
          <span className={cn("text-xs", className)} {...rest}>
            {address}
          </span>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  )
}

export default Address
