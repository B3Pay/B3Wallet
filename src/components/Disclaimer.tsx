import { Alert, AlertIcon, Box, Heading, Text } from "@chakra-ui/react"
import React from "react"

interface DisclaimerProps {
  noTitle?: boolean
}

const Disclaimer: React.FC<DisclaimerProps> = ({ noTitle }) => {
  return (
    <Box maxWidth="800px" margin="auto" padding="20px">
      <Heading as="h3" fontSize="2xl" marginBottom="10px" hidden={noTitle}>
        Disclaimer
      </Heading>
      <Alert status="warning" marginBottom="10px">
        <AlertIcon />
        <Text>
          The B3Wallet web wallet is currently in an alpha version and is not
          yet fully tested or audited. Use of this wallet is at your own risk.
        </Text>
      </Alert>
      <Alert status="warning" marginBottom="10px">
        <AlertIcon />
        <Text>
          B3Wallet and its developers make no warranty or representation, either
          express or implied, with respect to the software, including its
          quality, performance, merchantability, or fitness for a particular
          purpose. In no event will B3Wallet or its developers be liable for any
          direct, indirect, consequential, incidental, special, or exemplary
          damages, including but not limited to lost profits, lost savings, or
          lost data, arising out of the use or inability to use this software.
        </Text>
      </Alert>
      <Alert status="warning" marginBottom="10px">
        <AlertIcon />
        <Text>
          Cryptocurrency transactions are irreversible. If you send
          cryptocurrency to the wrong address, or if your wallet is compromised
          due to security vulnerabilities, you may lose your funds permanently.
          Always double-check addresses and transaction details before sending
          cryptocurrency.
        </Text>
      </Alert>
      <Alert status="warning" marginBottom="10px">
        <AlertIcon />
        <Text>
          The volatility and unpredictability of cryptocurrency prices can lead
          to significant financial losses. We recommend using this wallet only
          with testnet tokens until you are familiar with how it works.
        </Text>
      </Alert>
      <Alert status="warning" marginBottom="10px">
        <AlertIcon />
        <Text>
          This wallet does not provide any investment advice or recommendations.
          It is your responsibility to educate yourself about the risks of
          cryptocurrency and to make your financial decisions carefully.
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
    </Box>
  )
}

export default Disclaimer
