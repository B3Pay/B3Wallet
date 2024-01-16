import { useFormContext } from "react-hook-form"
import { CandidRouteProps } from "../route"
import { InfoCircledIcon } from "@radix-ui/react-icons"
import {
  FormItem,
  FormLabel,
  FormControl,
  FormMessage
} from "@src/components/ui/form"
import { Input } from "@src/components/ui/input"

export interface NumberProps extends CandidRouteProps<"number"> {}

const Number: React.FC<NumberProps> = ({
  registerName,
  errors,
  extractedField,
  shouldUnregister
}) => {
  const { resetField, register, trigger } = useFormContext()

  const resetHandler = () => {
    resetField(registerName as never)
    trigger(registerName as never, { shouldFocus: true })
  }

  const errorMessage = errors?.message?.toString()

  return (
    <FormItem>
      <FormLabel>{extractedField.label.toTitleCase()}</FormLabel>
      <FormControl>
        <Input
          id={registerName}
          {...register(registerName, {
            ...extractedField,
            shouldUnregister
          })}
          icon={<InfoCircledIcon />}
          closeHandler={resetHandler}
          type="number"
          placeholder={extractedField.type}
        />
      </FormControl>
      <FormMessage>{errorMessage}</FormMessage>
    </FormItem>
  )
}

export { Number }
