import { Label } from "@src/components/ui/label"
import { Route, RouteProps } from "."

export interface RecordProps extends RouteProps<"record"> {}

const Record: React.FC<RecordProps> = ({
  extractedField,
  errors,
  registerName
}) => {
  return (
    <div className="w-full">
      <Label>{extractedField.label}</Label>
      <div className="ml-3">
        {extractedField.fields.map((field, index) => (
          <Route
            key={index}
            registerName={`${registerName}.${field.label}`}
            extractedField={field}
            errors={errors?.[field.label as never]}
          />
        ))}
      </div>
    </div>
  )
}

export { Record }
