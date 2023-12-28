import { useFormContext } from "react-hook-form"
import { FieldRouteProps } from "./FieldRoute"
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "components/ui/form"
import { Input } from "components/ui/input"
import {
  CheckCircledIcon,
  CircleBackslashIcon,
  InfoCircledIcon,
  QuestionMarkCircledIcon
} from "@radix-ui/react-icons"
import { Box } from "components/ui/box"

interface FieldInputProps extends FieldRouteProps {}

const FieldInput: React.FC<FieldInputProps> = ({
  registerName,
  methodField
}) => {
  const { control, resetField } = useFormContext()

  const validate = (x: any) => {
    if (methodField.type === "null") {
      return methodField.validate(null)
    } else {
      return methodField.validate(x)
    }
  }

  return methodField.type !== "null" ? (
    <Box className="mb-4">
      <FormField
        control={control}
        name={registerName}
        rules={{
          ...methodField,
          validate
        }}
        render={({ field }) => (
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
                {...field}
                closeHandler={() => {
                  resetField(registerName as never)
                }}
              />
            </FormControl>
            <FormMessage />
          </FormItem>
        )}
      />
    </Box>
  ) : null
}

export default FieldInput
