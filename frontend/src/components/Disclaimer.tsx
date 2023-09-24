import { Alert, AlertIcon, Heading, Stack, Text } from "@chakra-ui/react"

interface DisclaimerProps {
  noTitle?: boolean
  asCard?: boolean
}

const Disclaimer: React.FC<DisclaimerProps> = ({
  noTitle,
  asCard: withCard
}) => {
  return (
    <Stack
      backgroundColor={withCard ? "white" : "transparent"}
      marginTop={withCard ? 2 : 0}
      borderRadius={withCard ? "md" : 0}
      padding={0}
      spacing={0}
    >
      <Heading
        as="h3"
        fontSize="2xl"
        marginBottom="10px"
        hidden={noTitle}
        padding={2}
        mb={2}
        alignItems="center"
        alignContent="center"
        borderBottom="1px solid"
        borderColor="gray.200"
      >
        Disclaimer
      </Heading>
      <Stack padding={2} spacing={2}>
        <Alert status="warning" marginBottom="10px">
          <AlertIcon />
          <Text>
            The B3Wallet web wallet is currently in an alpha version and is not
            yet fully tested or audited. Use of this wallet is at your own risk.
            We recommend using this wallet only with testnet tokens
          </Text>
        </Alert>
        <Alert status="warning" marginBottom="10px">
          <AlertIcon />
          <Text>
            B3Wallet and its developers make no warranty or representation,
            either express or implied, with respect to the software, including
            its quality, performance, merchantability, or fitness for a
            particular purpose. In no event will B3Wallet or its developers be
            liable for any direct, indirect, consequential, incidental, special,
            or exemplary damages, including but not limited to lost profits,
            lost savings, or lost data, arising out of the use or inability to
            use this software.
          </Text>
        </Alert>
        <Alert status="warning">
          <AlertIcon />
          <Text>
            By using B3Wallet, you agree to these terms and assume all risks
            associated with your use of the wallet. Please be safe and exercise
            caution when dealing with cryptocurrencies.
          </Text>
        </Alert>
      </Stack>
    </Stack>
  )
}

export default Disclaimer
