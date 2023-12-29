import React, { useEffect, useRef } from "react"
import FieldRoute, { FieldRouteProps } from "./FieldRoute"
import { useFormContext } from "react-hook-form"
import {
  Select,
  SelectItem,
  SelectContent,
  SelectGroup,
  SelectTrigger,
  SelectValue
} from "components/ui/select"
import { FormItem, FormLabel } from "components/ui/form"

interface VariantProps extends FieldRouteProps {}

const Variant: React.FC<VariantProps> = ({
  methodField,
  registerName,
  errors
}) => {
  const { unregister, setValue, resetField } = useFormContext()
  const selectedRef = useRef<string>()

  const changeHandler = (inputValue: string) => {
    const select = selectedRef.current

    resetField(`${registerName}.${select}`)
    unregister(registerName as never)
    setValue(
      registerName as never,
      { [inputValue]: methodField.defaultValues?.[inputValue] } as never
    )
    selectedRef.current = inputValue
  }

  const selectedField = methodField.fields?.find(
    methodField => methodField.label === selectedRef.current
  )

  return (
    <div>
      <FormItem>
        <FormLabel>{methodField.label}</FormLabel>
        <Select onValueChange={changeHandler}>
          <SelectTrigger className="w-full">
            <SelectValue placeholder="Select" />
          </SelectTrigger>
          <SelectContent position="popper">
            {methodField.options?.map((label, index) => (
              <SelectItem key={index} value={label}>
                {label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </FormItem>
      {selectedField && (
        <FieldRoute
          registerName={`${registerName}.${selectedRef.current}`}
          errors={errors?.[selectedRef.current as never]}
          methodField={selectedField}
        />
      )}
    </div>
  )
}

export default Variant
