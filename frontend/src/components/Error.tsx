import {
  Alert,
  AlertDescription,
  AlertIcon,
  AlertProps,
  AlertTitle
} from "@chakra-ui/react"
import { compileError } from "helpers/utiles"
import { useMemo } from "react"

interface ErrorProps extends AlertProps {
  error: string
}

const Error: React.FC<ErrorProps> = ({ error, ...rest }) => {
  const { title, description } = useMemo(() => {
    const errors = error
      ? error.toString().includes("Error::")
        ? error.toString().split("Error::")
        : [error.toString()]
      : ["Unknown error"]

    return compileError(errors)
  }, [error])

  return (
    <Alert status="error" {...rest}>
      <AlertIcon />
      <AlertTitle>{title}</AlertTitle>
      <AlertDescription>{description}</AlertDescription>
    </Alert>
  )
}

export default Error
