import { useFormContext } from "react-hook-form"
import { FieldSwitchProps } from "./FieldSwitch"
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

interface FieldInputProps extends FieldSwitchProps {}

const FieldInput: React.FC<FieldInputProps> = ({
  registerName,
  methodField
}) => {
  const { control, getFieldState, resetField } = useFormContext()

  const validate = (x: any) => {
    if (methodField.type === "null") {
      return methodField.validate(null)
    } else {
      return methodField.validate(x)
    }
  }

  const { isTouched, invalid, isDirty, error } = getFieldState(
    registerName as never
  )
  console.log("error", error)
  return methodField.type !== "null" ? (
    <div className="mb-4">
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
    </div>
  ) : null
}

export default FieldInput
