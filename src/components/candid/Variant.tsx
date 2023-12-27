import React, { useEffect, useRef } from "react"
import FieldSwitch, { FieldSwitchProps } from "./FieldSwitch"
import { useFormContext } from "react-hook-form"
import {
  Select,
  SelectContent,
  SelectTrigger,
  SelectValue
} from "components/ui/select"
import { SelectItem } from "@radix-ui/react-select"

interface VariantProps extends FieldSwitchProps {}

const Variant: React.FC<VariantProps> = ({
  methodField: field,
  registerName,
  errors
}) => {
  const { unregister, setValue, resetField } = useFormContext()
  const selectedRef = useRef<string>(field.options?.[0] as string)

  const changeHandler = (inputValue: string) => {
    const select = selectedRef.current

    resetField(`${registerName}.${select}`)
    unregister(registerName as never)
    setValue(
      registerName as never,
      { [inputValue]: field.defaultValues?.[inputValue] } as never
    )
    selectedRef.current = inputValue
  }

  const selectedField = field.fields?.find(
    field => field.label === selectedRef.current
  )

  return (
    <Select onValueChange={changeHandler} defaultValue={field.options?.[0]}>
      <SelectTrigger className="w-full">
        <SelectValue placeholder="Select" />
      </SelectTrigger>
      <SelectContent>
        {field.options?.map((label, index) => (
          <SelectItem key={index} value={label}>
            {label}
          </SelectItem>
        ))}
      </SelectContent>
      {selectedField ? (
        <FieldSwitch
          registerName={`${registerName}.${selectedRef.current}`}
          errors={errors?.[selectedRef.current as never]}
          methodField={selectedField}
        />
      ) : (
        <div className="mt-2">Field not found</div>
      )}
    </Select>
  )
}

export default Variant
