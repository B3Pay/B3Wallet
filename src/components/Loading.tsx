import { CircularProgress, Flex } from "@chakra-ui/react"
import React from "react"

interface LoadingProps {
  dark?: boolean
}

const Loading: React.FC<LoadingProps> = ({ dark }) => {
  return (
    <Flex
      position="absolute"
      top="0"
      bottom="0"
      left="0"
      right="0"
      align="center"
      justify="center"
      zIndex={1000}
      backdropBrightness={dark ? "dark" : "light"}
      backdropFilter="blur(10px) hue-rotate(90deg)"
    >
      <CircularProgress isIndeterminate color="green.300" />
    </Flex>
  )
}

export default Loading
