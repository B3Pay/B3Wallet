import { Box, Link, VStack } from "@chakra-ui/react"
import React from "react"

interface FooterProps {}

export const Footer: React.FC<FooterProps> = ({}) => {
  return (
    <Box as="footer" role="contentinfo" py="2" px={{ base: "4", md: "8" }}>
      <VStack>
        <Box fontSize="sm">
          &copy; {new Date().getFullYear()} B3Payment(
          {process.env.NEXT_PUBLIC_VERSION}
          ).
        </Box>
        {/* add github link */}
        <Link
          href="https://github.com/B3Pay/b3-wallet/tree/main/backend/b3_payment"
          isExternal
        >
          <Box fontSize="sm">Github</Box>
        </Link>
      </VStack>
    </Box>
  )
}
