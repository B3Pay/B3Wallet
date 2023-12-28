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
import { Box } from "components/ui/box"

interface FieldInputProps extends FieldRouteProps {}

const FieldInput: React.FC<FieldInputProps> = ({
  registerName,
  methodField
}) => {
  const { register, resetField } = useFormContext()

  const validate = (x: any) => {
    if (methodField.type === "null") {
      return methodField.validate(null)
    } else {
      return methodField.validate(x)
    }
  }

  return methodField.type !== "null" ? (
    <Box className="mb-4">
      <FormItem>
        <FormLabel>
          {methodField.label}
          {methodField.required && <span className="text-red-500">*</span>}
        </FormLabel>
        <FormControl>
          <Input
            icon={<InfoCircledIcon />}
            type={methodField.type}
            placeholder={methodField.type}
            closeHandler={() => {
              resetField(registerName as never)
            }}
            {...register(registerName, {
              ...methodField,
              validate
            })}
          />
        </FormControl>
        <FormMessage />
      </FormItem>
    </Box>
  ) : null
}

export default FieldInput
