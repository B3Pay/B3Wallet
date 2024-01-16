import { Label } from "@src/components/ui/label"
import { CandidRoute, CandidRouteProps } from "."

export interface TupleProps extends CandidRouteProps<"tuple"> {}

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
        <CandidRoute
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
