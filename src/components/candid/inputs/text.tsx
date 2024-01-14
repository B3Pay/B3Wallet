import { useFormContext } from "react-hook-form"
import { RouteProps } from "../route"
import { InfoCircledIcon } from "@radix-ui/react-icons"
import {
  FormItem,
  FormLabel,
  FormControl,
  FormMessage,
  FormField
} from "@src/components/ui/form"
import { Input } from "@src/components/ui/input"

export interface TextProps extends RouteProps {}

const Text: React.FC<TextProps> = ({
  registerName,
  errors,
  extractedField,
  shouldUnregister
}) => {
  const { resetField, trigger } = useFormContext()

  const resetHandler = () => {
    resetField(registerName as never)
    trigger(registerName as never, { shouldFocus: true })
  }

  const errorMessage = errors?.message?.toString()

  return (
    <FormItem>
      <FormLabel>{extractedField.label.toTitleCase()}</FormLabel>
      <FormControl>
        <FormField
          name={registerName}
          shouldUnregister={shouldUnregister}
          defaultValue={extractedField.defaultValue}
          rules={extractedField}
          render={({ field }) => (
            <Input
              {...field}
              icon={<InfoCircledIcon />}
              closeHandler={resetHandler}
              id={registerName}
              type="text"
              placeholder={extractedField.type}
            />
          )}
        />
      </FormControl>
      <FormMessage>{errorMessage}</FormMessage>
    </FormItem>
  )
}

export { Text }
