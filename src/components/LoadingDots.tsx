import { Box } from "@chakra-ui/react"
import { useEffect, useState } from "react"

const LoadingDots = () => {
  const [dots, setDots] = useState(".")

  useEffect(() => {
    const timer = setInterval(() => {
      setDots(dots => (dots.length < 3 ? dots + "." : "."))
    }, 500)

    return () => clearInterval(timer)
  }, [])

  return <Box>{`Loading${dots}`}</Box>
}

export default LoadingDots
