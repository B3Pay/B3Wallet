import { Card, Heading } from "@chakra-ui/react"

interface HeaderProps {}

const Header: React.FC<HeaderProps> = ({}) => {
  return (
    <Card mb={2}>
      <Heading textAlign="center" p={4} my={2}>
        B3Wallet Demo
      </Heading>
    </Card>
  )
}

export default Header
