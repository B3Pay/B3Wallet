import { CheckIcon, CopyIcon } from "@chakra-ui/icons"
import {
  Flex,
  FlexProps,
  IconButton,
  Text,
  Tooltip,
  useClipboard,
  useMediaQuery
} from "@chakra-ui/react"
import { useMemo } from "react"

interface AddressWithCopyProps extends FlexProps {
  address: string
  noIcon?: boolean
  hiddenAddress?: boolean
}

const Address: React.FC<AddressWithCopyProps> = ({
  address,
  noIcon,
  overflow,
  children,
  hiddenAddress,
  ...rest
}) => {
  const { hasCopied, onCopy } = useClipboard(address)
  const [isLargerThan500] = useMediaQuery(["(min-width: 568px)"])

  const truncatedAddress = useMemo(() => {
    if (
      overflow ||
      address.length <= 20 ||
      (isLargerThan500 && address.length <= 42)
    ) {
      return address
    }

    const Start = address.slice(0, isLargerThan500 ? 20 : 8)
    const End = address.slice(isLargerThan500 ? -20 : -8)

    return `${Start}...${End}`
  }, [address, overflow, isLargerThan500])

  return (
    <Tooltip label={address} aria-label="Full address">
      <Flex alignItems="center" overflow={overflow} {...rest}>
        {children}
        <Text
          overflow="hidden"
          textOverflow="ellipsis"
          whiteSpace="nowrap"
          hidden={hiddenAddress}
        >
          {truncatedAddress}
        </Text>
        {noIcon ? null : (
          <IconButton
            colorScheme="blue"
            onClick={onCopy}
            aria-label="Copy to clipboard"
            variant={hiddenAddress ? "solid" : "ghost"}
            icon={hasCopied ? <CheckIcon /> : <CopyIcon />}
          />
        )}
      </Flex>
    </Tooltip>
  )
}

export default Address
