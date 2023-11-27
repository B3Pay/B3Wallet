import {
  Box,
  Card,
  HStack,
  Link,
  Text,
  VStack,
  keyframes
} from "@chakra-ui/react"

const SpinY = keyframes`
  0% {
    transform: rotateY(0deg);
  }
  25% {
    transform: rotateY(0deg);
  }
  50% {
    transform: rotateY(180deg);
  }
  75% {
    transform: rotateY(360deg);
  }
  100% {
    transform: rotateY(360deg);
  }
`

interface FooterProps {}

const Footer: React.FC<FooterProps> = ({}) => {
  return (
    <HStack mt={2} height={12}>
      <Card px={2} py={1} height="100%" width="96px">
        <Link
          href="https://internetcomputer.org/"
          target="_blank"
          rel="noopener"
        >
          <VStack spacing={1}>
            <Box
              as="img"
              width={12}
              height={6}
              src="/icp.svg"
              alt="icp-logo"
              animation={`${SpinY} 5s ease-in-out infinite`}
            />
            <Box whiteSpace="nowrap" fontFamily="Monospace" fontSize={8}>
              100% on-chain
            </Box>
          </VStack>
        </Link>
      </Card>
      <Card
        height="100%"
        width="calc(100% - 160px)"
        justify="center"
        sx={{
          textAlign: "center",
          color: "text.secondary",
          p: 1
        }}
      >
        <Text variant="caption" fontSize="sm" px={{ base: "4", md: "8" }}>
          &copy; {new Date().getFullYear()} B3Wallet Demo (v
          {process.env.NEXT_PUBLIC_VERSION || "0.0.0"})
        </Text>
      </Card>
      <Card px={2} py={1} height="100%" justify="center" width="64px">
        <Link
          href="https://github.com/B3Pay/b3-wallet"
          target="_blank"
          rel="noopener"
        >
          <Box
            as="img"
            width={8}
            height={8}
            margin="auto"
            src="/gh-logo.png"
            alt="github-logo"
          />
        </Link>
      </Card>
    </HStack>
  )
}

export default Footer
