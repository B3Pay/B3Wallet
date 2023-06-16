import { Box, Stack, Text } from "@chakra-ui/react"
import { WalletSetting } from "declarations/b3_wallet/b3_wallet.did"
import { B3Wallet } from "service/actor"
import Address from "../Address"

interface ControllersProps extends WalletSetting {
  actor: B3Wallet
}

const Controllers: React.FC<ControllersProps> = ({ metadata, controllers }) => {
  return (
    <Stack spacing={4}>
      <Box bg="gray.100" p={4} borderRadius="md">
        <Text fontSize="lg" fontWeight="bold">
          Metadata
        </Text>
      </Box>
      {Object.entries(metadata).map((controller, index) => (
        <Box key={index} bg="gray.100" p={4} borderRadius="md">
          <Address address={controller.toString()} noIcon />
          <Text fontSize="sm">{controller[1]}</Text>
        </Box>
      ))}
      <Box bg="gray.100" p={4} borderRadius="md">
        <Text fontSize="lg" fontWeight="bold">
          Controllers
        </Text>
      </Box>
      {controllers.map((controller, index) => (
        <Box key={index} bg="gray.100" p={4} borderRadius="md">
          <Text fontSize="lg" fontWeight="bold">
            {controller.name}
          </Text>
          <Text fontSize="sm">{controller.address}</Text>
          <Text fontSize="sm">{controller.public_key}</Text>
        </Box>
      ))}
    </Stack>
  )
}

export default Controllers
