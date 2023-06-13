import { ToastProps, useToast } from "@chakra-ui/react"
import { useCallback } from "react"

const compileError = (description: string[]) => {
  if (description.length > 1) {
    return {
      title: description[1],
      description: description[2]
    }
  } else {
    return {
      title: "Error",
      description: description[0]
    }
  }
}

const useToastMessage = () => {
  const toast = useToast()

  const errorToast = useCallback(
    (props: ToastProps) => {
      const errors = props.description
        ? props.description.toString().includes("Error::")
          ? props.description.toString().split("Error::")
          : [props.description.toString()]
        : ["Unknown error"]

      const { title, description } = compileError(errors)

      return toast({
        ...props,
        title,
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

  return errorToast
}

export default useToastMessage
