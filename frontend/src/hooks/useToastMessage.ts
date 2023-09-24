import { ToastProps, useToast } from "@chakra-ui/react"
import { compileError } from "helpers/utiles"
import { useCallback } from "react"

const useToastMessage = () => {
  const toast = useToast()

  const errorToast = useCallback(
    (props: ToastProps) => {
      const errors = props.description
        ? props.description.toString().includes("Error::")
          ? props.description.toString().split("Error::")
          : [props.description.toString()]
        : ["Unknown error"]

      const { title, description } = compileError(errors, props.title)
      console.log("errorToast -> title", title, description)
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
