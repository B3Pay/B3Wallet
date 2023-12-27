import { useFieldArray, useFormContext } from "react-hook-form"
import FormField, { FormFieldsProps } from "./FormField"
import { cn } from "lib/utils"

interface OptionalProps extends FormFieldsProps {}

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
          <input
            id={registerName}
            className="hidden"
            type="checkbox"
            onClick={() => (fields.length === 0 ? append("") : remove(0))}
          />
          <label
            htmlFor={registerName}
            className={cn(
              "relative inline-block w-12 h-6 rounded-full cursor-pointer transition duration-200",
              fields.length > 0 ? "bg-green-400" : "bg-gray-600"
            )}
          >
            <span
              className={cn(
                "absolute left-1 top-1 w-4 h-4 bg-white rounded-full transition-transform transform",
                fields.length > 0 ? "translate-x-6" : "translate-x-0"
              )}
            />
          </label>
        </div>
      </div>
      {fields.length > 0 && (
        <div className="flex justify-between items-start p-1 mb-1 w-full border-dashed border border-gray-400 rounded">
          <FormField
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
