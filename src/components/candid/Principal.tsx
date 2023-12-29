import { Principal as PrincipalId } from "@dfinity/principal"
import { useFormContext } from "react-hook-form"
import { FieldRouteProps } from "./FieldRoute"
import {
  FormControl,
  FormItem,
  FormLabel,
  FormMessage
} from "components/ui/form"
import { Input } from "components/ui/input"
import { InfoCircledIcon } from "@radix-ui/react-icons"

interface PrincipalProps extends FieldRouteProps {}

const Principal: React.FC<PrincipalProps> = ({
  registerName,
  errors,
  methodField
}) => {
  const { setValue, register, resetField, setError } = useFormContext()

  const blurHandler = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.value === "") {
      setValue(registerName as never, "" as never)
      return
    }
    const inputValue = e.target.value
    resetField(registerName as never, { defaultValue: inputValue as never })
    const isValid = validate(inputValue)

    if (isValid === true) {
      const principal = PrincipalId.fromText(inputValue)

      setValue(registerName as never, principal as never)
    } else {
      setError(registerName as never, {
        type: "manual",
        message: isValid
      })
    }
  }

  function validate(x: any) {
    if (x._isPrincipal === true) {
      return true
    }
    try {
      if (x.length < 7) {
        throw new Error("Principal is too short")
      }
      const principal = PrincipalId.fromText(x)

      const validate = methodField.validate(principal)

      if (typeof validate === "string") {
        throw new Error(validate)
      }
      return true
    } catch (error) {
      return (error as any).message
    }
  }

  const errorMessage = errors?.message?.toString()

  return (
    <FormItem>
      <FormLabel>{methodField.label.toTitleCase()}</FormLabel>
      <FormControl>
        <Input
          icon={<InfoCircledIcon />}
          type={methodField.type}
          placeholder={methodField.type}
          closeHandler={() => {
            setValue(registerName as never, "" as never)
          }}
          {...register(registerName, {
            ...methodField,
            validate
          })}
          onBlur={blurHandler}
        />
      </FormControl>
      <FormMessage>{errorMessage}</FormMessage>
    </FormItem>
  )
}

export default Principal
