import React, { useId } from "react"
import { Controller, useWatch } from "react-hook-form"
import { Route, RouteProps } from "."
import {
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "@src/components/ui/form"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from "@src/components/ui/select"

export interface VariantProps extends RouteProps<"variant"> {}

const Variant: React.FC<VariantProps> = ({
  extractedField,
  registerName,
  errors
}) => {
  const selectRegisterName = useId()

  const selectedOption = useWatch({
    name: selectRegisterName,
    defaultValue: extractedField.defaultValue
  })

  return (
    <div className="w-full flex-col">
      <FormItem>
        <FormLabel>{extractedField.label.toTitleCase()}</FormLabel>
        <Controller
          name={selectRegisterName}
          defaultValue={extractedField.defaultValue}
          render={({ field }) => (
            <Select
              value={field.value || "select"}
              onValueChange={field.onChange}
            >
              <SelectTrigger className="w-full">
                <SelectValue placeholder={extractedField.defaultValue} />
              </SelectTrigger>
              <SelectContent position="popper">
                {extractedField.options.map((label, index) => (
                  <SelectItem key={index} value={label}>
                    {label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          )}
        />
      </FormItem>
      <FormMessage />
      {extractedField.fields.map(
        (field, index) =>
          selectedOption === field.label && (
            <Route
              key={index}
              shouldUnregister
              extractedField={field}
              registerName={`${registerName}.${field.label}`}
              errors={errors?.[field.label as never]}
            />
          )
      )}
    </div>
  )
}

export { Variant }
