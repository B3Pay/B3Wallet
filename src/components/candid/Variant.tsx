import React, { useMemo, useRef } from "react"
import FieldRoute, { FieldRouteProps } from "./FieldRoute"
import { useFormContext, useWatch } from "react-hook-form"
import {
  Select,
  SelectItem,
  SelectContent,
  SelectTrigger,
  SelectValue
} from "@src/components/ui/select"
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "@src/components/ui/form"

interface VariantProps extends FieldRouteProps {}

let recursiveCounter = 0

const Variant: React.FC<VariantProps> = ({
  methodField,
  registerName,
  errors,
  ...rest
}) => {
  const { control, unregister, setValue } = useFormContext()
  const [selectedName, setSelectedName] = React.useState<string>()

  const selectName = useMemo(() => `select.select${recursiveCounter++}`, [])

  const selected = useWatch({ name: selectName })

  React.useEffect(() => {
    if (selected) {
      unregister(registerName)
      setValue(
        `${registerName}.${selected}`,
        methodField.defaultValues?.[selected]
      )
      setSelectedName(`${registerName}.${selected}`)
    }
  }, [selected])

  const selectedField = useMemo(() => {
    return selected ? methodField.fields.find(f => f.label === selected) : null
  }, [selected])

  return (
    <div>
      <FormField
        name={selectName}
        control={control}
        rules={{
          required: true,
          validate: value => (value === "select" ? "Please select one" : true)
        }}
        render={({ field }) => (
          <FormItem>
            <FormLabel>{methodField.label.toTitleCase()}</FormLabel>
            <Select
              {...field}
              onValueChange={field.onChange}
              value={field.value || "select"}
            >
              <FormControl>
                <SelectTrigger className="w-full">
                  <SelectValue placeholder="Select" />
                </SelectTrigger>
              </FormControl>
              <SelectContent position="popper">
                <SelectItem
                  value="select"
                  disabled
                  style={{
                    display: "none"
                  }}
                >
                  Select
                </SelectItem>
                {methodField.options?.map((label, index) => (
                  <SelectItem key={index} value={label}>
                    {label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </FormItem>
        )}
      />
      <FormMessage />
      {selectedField && selectedName && (
        <FieldRoute
          registerName={selectedName}
          errors={errors?.[selected as never]}
          methodField={selectedField}
          {...rest}
        />
      )}
    </div>
  )
}

export default Variant
