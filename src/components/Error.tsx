import { Alert, AlertIcon } from "@chakra-ui/react"
import { useMemo } from "react"

interface ErrorProps {
  error: string
}

const Error: React.FC<ErrorProps> = ({ error }) => {
  const parsedError = useMemo(
    () =>
      error.toString().includes("::")
        ? error.toString().split("::")[1].trim()
        : error.toString(),
    [error]
  )

  return (
    <Alert status="error">
      <AlertIcon />
      {parsedError}
    </Alert>
  )
}

export default Error
