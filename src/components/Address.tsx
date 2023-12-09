import { CheckIcon, CopyIcon } from "@radix-ui/react-icons"
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger
} from "components/ui/tooltip"
import { useMemo, useState } from "react"
import { Button } from "./ui/button"

interface AddressWithCopyProps {
  address: string
  noIcon?: boolean
  smallest?: boolean
  hiddenAddress?: boolean
}

const Address: React.FC<AddressWithCopyProps> = ({
  address,
  noIcon,
  smallest,
  hiddenAddress
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

    const Start = address.slice(0, isLargerThan500 ? (smallest ? 13 : 20) : 8)
    const End = address.slice(isLargerThan500 ? (smallest ? -13 : -20) : -8)

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
          <p className="text-xs">{address}</p>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  )
}

export default Address
