import { useState, useEffect } from "react"
import { Route, RouteProps } from "."
import { DynamicFieldType } from "@ic-reactor/react/dist/types"

export interface RecursiveProps extends RouteProps<"recursive"> {}

const Recursive: React.FC<RecursiveProps> = ({
  extractedField,
  errors,
  registerName
}) => {
  const [recursiveField, setRecursiveFields] = useState<DynamicFieldType>()

  useEffect(() => {
    const fields = extractedField.extract()
    setRecursiveFields(fields)
  }, [extractedField])

  return recursiveField ? (
    <Route
      extractedField={recursiveField}
      registerName={registerName}
      errors={errors}
    />
  ) : (
    <div>Loading...</div>
  )
}
export { Recursive }
