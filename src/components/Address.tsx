import { CheckIcon, CopyIcon } from "@chakra-ui/icons"
import {
  Flex,
  IconButton,
  Tooltip,
  useClipboard,
  useMediaQuery
} from "@chakra-ui/react"
import { useMemo } from "react"

interface AddressWithCopyProps {
  address: string
}

const Address: React.FC<AddressWithCopyProps> = ({ address }) => {
  const { hasCopied, onCopy } = useClipboard(address)
  const [isLargerThan500] = useMediaQuery(["(min-width: 500px)"])

  const truncatedAddress = useMemo(() => {
    if (isLargerThan500 && address.length <= 42) {
      return address
    }

    const Start = address.slice(0, isLargerThan500 ? 20 : 8)
    const End = address.slice(isLargerThan500 ? -20 : -8)

    return `${Start}...${End}`
  }, [address, isLargerThan500])

  return (
    <Tooltip label={address} aria-label="Full address">
      <Flex alignItems="center">
        {truncatedAddress}
        <IconButton
          colorScheme="blue"
          onClick={onCopy}
          aria-label="Copy to clipboard"
          variant="ghost"
          size="sm"
          icon={hasCopied ? <CheckIcon /> : <CopyIcon />}
        />
      </Flex>
    </Tooltip>
  )
}

export default Address
