import { CandidRouteProps } from "../route"
import { FormField, FormItem } from "@src/components/ui/form"

export interface BooleanProps extends CandidRouteProps<"boolean"> {}

const Boolean: React.FC<BooleanProps> = ({
  extractedField,
  shouldUnregister,
  registerName
}) => {
  return (
    <FormItem>
      <label htmlFor={registerName} className="block mr-2">
        {extractedField.label}
      </label>
      <FormField
        shouldUnregister={shouldUnregister}
        name={registerName}
        defaultValue={false as never}
        render={({ field }) => (
          <input
            {...field}
            id={registerName}
            className="h-4 w-4 ml-4 border rounded"
            type="checkbox"
          />
        )}
      />
    </FormItem>
  )
}

export { Boolean }
