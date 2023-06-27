import { Stack, StackProps, Text } from "@chakra-ui/react"
import Address from "./Address"

interface PrincipalCardProps extends StackProps {
  address: string
}

const PrincipalCard: React.FC<PrincipalCardProps> = ({ address, ...rest }) => {
  return (
    <Stack
      p={4}
      align="center"
      direction="row"
      borderWidth="1px"
      borderRadius="lg"
      overflow="hidden"
      position="relative"
      justifyContent="space-between"
      {...rest}
    >
      <Text flex={6} fontSize="md" fontWeight="bold">
        Your Principal
      </Text>
      <Address address={address} px={2} />
    </Stack>
  )
}

export default PrincipalCard
