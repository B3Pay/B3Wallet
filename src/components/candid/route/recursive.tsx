import { useState, useEffect } from "react"
import { CandidRoute, CandidRouteProps } from "."
import { DynamicFieldType } from "@ic-reactor/store"

export interface RecursiveProps extends CandidRouteProps<"recursive"> {}

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
    <CandidRoute
      extractedField={recursiveField}
      registerName={registerName}
      errors={errors}
    />
  ) : (
    <div>Loading...</div>
  )
}
export { Recursive }
