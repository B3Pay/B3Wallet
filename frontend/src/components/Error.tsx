import { Alert, AlertIcon } from "@chakra-ui/react"
import { useMemo } from "react"

interface ErrorProps {
  error: string
}

const Error: React.FC<ErrorProps> = ({ error }) => {
  const parsedError = useMemo(
    () =>
      error.toString().includes("Error::")
        ? error.toString().split("Error::")
        : [error.toString()],
    [error]
  )

  return (
    <Alert status="error">
      <AlertIcon />
      {parsedError.map((err, index) => (
        <p key={index}>{err}</p>
      ))}
    </Alert>
  )
}

export default Error
