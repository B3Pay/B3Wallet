import { CheckIcon, CopyIcon } from "@radix-ui/react-icons"
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger
} from "components/ui/tooltip"
import { cn } from "lib/utils"
import { useMemo, useState } from "react"
import { Button } from "./ui/button"

interface AddressWithCopyProps
  extends React.HTMLAttributes<HTMLParagraphElement> {
  address: string
  noIcon?: boolean
  hiddenAddress?: boolean
  size?: "xs" | "sm" | "md" | "lg" | "xl" | null
}

const Address: React.FC<AddressWithCopyProps> = ({
  address,
  noIcon,
  hiddenAddress,
  className,
  size = "md",
  ...rest
}) => {
  const [hasCopied, setHasCopied] = useState(false)
  const [isLargerThan500, setIsLargerThan500] = useState(
    window.innerWidth > 568
  )

  const onCopy = () => {
    navigator.clipboard.writeText(address)
    setHasCopied(true)
    setTimeout(() => setHasCopied(false), 2000)
  }

  window.addEventListener("resize", () => {
    setIsLargerThan500(window.innerWidth > 568)
  })

  const truncatedAddress = useMemo(() => {
    if (address.length <= 20 || (isLargerThan500 && address.length <= 42)) {
      return address
    }

    const maxLength = size === "xs" ? 8 : size === "sm" ? 12 : 20

    const Start = address.slice(0, isLargerThan500 ? maxLength : 8)
    const End = address.slice(isLargerThan500 ? -maxLength : -8)

    return `${Start}...${End}`
  }, [address, isLargerThan500])

  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <div className="flex items-center overflow-hidden" title={address}>
            <p
              className={`overflow-hidden overflow-ellipsis whitespace-nowrap ${
                hiddenAddress ? "hidden" : ""
              }`}
            >
              {truncatedAddress}
            </p>
            {!noIcon && (
              <Button
                variant="link"
                onClick={onCopy}
                asIconButton
                aria-label="Copy to clipboard"
              >
                {hasCopied ? (
                  <CheckIcon className="h-5 w-5" />
                ) : (
                  <CopyIcon className="h-5 w-5" />
                )}
              </Button>
            )}
          </div>
        </TooltipTrigger>
        <TooltipContent>
          <p className={cn("text-xs", className)} {...rest}>
            {address}
          </p>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  )
}

export default Address
