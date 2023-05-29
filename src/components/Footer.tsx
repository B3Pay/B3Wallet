import { Box, Flex, Link, VStack } from "@chakra-ui/react"
import React from "react"

interface FooterProps {}

export const Footer: React.FC<FooterProps> = ({}) => {
  return (
    <Box as="footer" role="contentinfo" py="2" px={{ base: "4", md: "8" }}>
      <VStack>
        <Flex alignItems="center">
          <Link href="https://github.com/B3Pay/b3-wallet" isExternal>
            GitHub
          </Link>
        </Flex>
        <Box fontSize="sm">&copy; {new Date().getFullYear()} B3Wallet.</Box>
      </VStack>
    </Box>
  )
}
