import { Alert, AlertIcon, Text } from "@chakra-ui/react"
import { compileError } from "helpers/utiles"
import { useMemo } from "react"

interface ErrorProps {
  error: string
}

const Error: React.FC<ErrorProps> = ({ error }) => {
  const { title, description } = useMemo(() => {
    const errors = error
      ? error.toString().includes("Error::")
        ? error.toString().split("Error::")
        : [error.toString()]
      : ["Unknown error"]

    return compileError(errors)
  }, [error])

  return (
    <Alert status="error">
      <AlertIcon />
      <Text fontWeight="bold">{title}</Text>
      <Text>{description}</Text>
    </Alert>
  )
}

export default Error
