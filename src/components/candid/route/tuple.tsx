import { Label } from "@src/components/ui/label"
import { Route, RouteProps } from "."

export interface TupleProps extends RouteProps<"tuple"> {}

const Tuple: React.FC<TupleProps> = ({
  extractedField,
  registerName,
  shouldUnregister,
  errors
}) => {
  return (
    <div className="w-full">
      <Label>{extractedField.label}</Label>
      {extractedField.fields.map((field, index) => (
        <Route
          key={index}
          registerName={`${registerName}.[${index}]`}
          errors={errors?.[index as never]}
          shouldUnregister={shouldUnregister}
          extractedField={field}
        />
      ))}
    </div>
  )
}

export { Tuple }
