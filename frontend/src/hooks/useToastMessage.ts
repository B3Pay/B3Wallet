import { ToastProps, useToast } from "@chakra-ui/react"
import { useCallback } from "react"

const useToastMessage = () => {
  const toast = useToast()

  const errorToast = useCallback(
    (props: ToastProps) => {
      const description = props.description
        ? props.description.toString().includes("::")
          ? props.description.toString().split("::")[1].trim()
          : props.description.toString()
        : ""

      return toast({
        ...props,
        description,
        duration: 100000,
        containerStyle: {
          zIndex: 9999,
          width: {
            base: "96%",
            md: "auto"
          }
        }
      })
    },
    [toast]
  )

  return { toast, errorToast }
}

export default useToastMessage
