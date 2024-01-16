import { useFieldArray } from "react-hook-form"
import { CandidRoute, CandidRouteProps } from "."
import { Switch } from "@src/components/ui/switch"
import { Box } from "@src/components/ui/box"
import { Label } from "@src/components/ui/label"

export interface OptionalProps extends CandidRouteProps<"optional"> {}

const Optional: React.FC<OptionalProps> = ({
  extractedField,
  registerName,
  shouldUnregister,
  errors
}) => {
  const { fields, insert, remove } = useFieldArray({
    name: registerName as never,
    shouldUnregister
  })

  return (
    <Box className="my-2">
      <Box className="flex justify-between items-center">
        <Label className="flex-1 w-full block text-lg font-medium">
          {extractedField.label.toTitleCase()}
        </Label>
        <Switch
          onCheckedChange={checked => {
            if (checked) {
              insert(0, extractedField.defaultValues)
            } else {
              remove(0)
            }
          }}
        />
      </Box>
      {fields.map((field, index) => (
        <CandidRoute
          key={field.id}
          extractedField={extractedField.field}
          errors={errors?.[index as never]}
          registerName={`${registerName}.[${index}]`}
        />
      ))}
    </Box>
  )
}
export { Optional }
