import { Box, CircularProgress, Progress, Stack, Text } from "@chakra-ui/react"
import React from "react"
import LoadingDots from "./LoadingDots"

interface LoadingProps {
  dark?: boolean
  circle?: boolean
  title?: string
}

const Loading: React.FC<LoadingProps> = ({ dark, circle, title }) => {
  return (
    <Stack
      position="absolute"
      top="0"
      bottom="0"
      left="0"
      right="0"
      align="center"
      justify="center"
      zIndex={1000}
      bg={dark ? "blackAlpha.800" : "whiteAlpha.800"}
      backdropFilter="blur(2px)"
    >
      <Text fontWeight="bold" color={dark ? "white" : "gray.600"}>
        <LoadingDots title={title} />
      </Text>
      <Box w="20vw">
        {circle ? (
          <CircularProgress isIndeterminate color="green.300" />
        ) : (
          <Progress size="xs" isIndeterminate />
        )}
      </Box>
    </Stack>
  )
}

export default Loading
