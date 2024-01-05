import React, { useMemo, useRef } from "react"
import FieldRoute, { FieldRouteProps } from "./FieldRoute"
import { useFormContext, useWatch } from "react-hook-form"
import {
  Select,
  SelectItem,
  SelectContent,
  SelectTrigger,
  SelectValue
} from "components/ui/select"
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "components/ui/form"

interface VariantProps extends FieldRouteProps {}

let recursiveCounter = 0

const Variant: React.FC<VariantProps> = ({
  methodField,
  registerName,
  errors
}) => {
  const currentRef = useRef<string>()
  const { control, unregister, setValue } = useFormContext()

  const selectName = useMemo(() => `select.select${recursiveCounter++}`, [])

  const selected = useWatch({ name: selectName })

  const { selectedName, selectedField } = useMemo(() => {
    if (!selected) {
      return {}
    }

    if (currentRef.current) unregister(registerName)

    const selectedName = `${registerName}.${selected}`

    setValue(selectedName, methodField.defaultValues?.[selected])

    const selectedField = methodField.fields.find(f => f.label === selected)

    currentRef.current = selectedName

    return { selectedName, selectedField }
  }, [selected, setValue])

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
      {selectedField && (
        <FieldRoute
          registerName={selectedName}
          errors={errors?.[selected as never]}
          methodField={selectedField}
        />
      )}
    </div>
  )
}

export default Variant
