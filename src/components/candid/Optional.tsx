import { useFieldArray, useFormContext } from "react-hook-form"
import FormFieldSwitch, { FormFieldSwitchProps } from "./FieldSwitch"
import { Switch } from "components/ui/switch"

interface OptionalProps extends FormFieldSwitchProps {}

const Optional: React.FC<OptionalProps> = ({ field, registerName, errors }) => {
  const { control } = useFormContext()

  const { fields, append, remove } = useFieldArray({
    control,
    name: registerName as never
  })

  return (
    <div className="flex flex-col">
      <div className="flex h-10 justify-between items-center">
        <label className="flex-1  w-full block text-lg font-medium">
          {field.label}
        </label>
        <div className="flex-auto w-18 mt-1">
          <Switch
            onChange={() => (fields.length === 0 ? append("") : remove(0))}
          />
        </div>
      </div>
      {fields.length > 0 && (
        <div className="flex justify-between items-start p-1 mb-1 w-full border-dashed border border-gray-400 rounded">
          <FormFieldSwitch
            field={field.fields?.[0]}
            registerName={`${registerName}.[0]`}
            errors={errors?.[0 as never]}
          />
        </div>
      )}
    </div>
  )
}
export default Optional
