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

interface FieldInputProps extends FieldRouteProps {}

const FieldInput: React.FC<FieldInputProps> = ({
  registerName,
  methodField,
  errors
}) => {
  const { register, resetField } = useFormContext()

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
            resetField(registerName as never)
          }}
          {...register(registerName, methodField)}
        />
      </FormControl>
      <FormMessage>{errorMessage}</FormMessage>
    </FormItem>
  )
}

export default FieldInput
