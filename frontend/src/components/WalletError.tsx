import {
  Alert,
  AlertDescription,
  AlertIcon,
  AlertProps,
  AlertTitle
} from "@chakra-ui/react"
import { compileError } from "helpers/utiles"
import { useMemo } from "react"

interface WalletErrorProps extends AlertProps {
  error: string
}

const WalletError: React.FC<WalletErrorProps> = ({ error, ...rest }) => {
  const { title, description } = useMemo(() => {
    const errors = error
      ? error.toString().includes("Error::")
        ? error.toString().split("Error::")
        : [error.toString()]
      : ["Unknown error"]

    return compileError(errors, "Error")
  }, [error])

  return (
    <Alert status="error" {...rest}>
      <AlertIcon />
      <AlertTitle>{title}</AlertTitle>
      <AlertDescription>{description}</AlertDescription>
    </Alert>
  )
}

export default WalletError
