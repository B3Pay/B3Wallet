import { Card, Heading } from "@chakra-ui/react"

interface HeaderProps {}

const Header: React.FC<HeaderProps> = ({}) => {
  return (
    <Card mb={2}>
      <Heading size="lg" textAlign="center" my={2}>
        B3Payment Demo
      </Heading>
    </Card>
  )
}

export default Header
