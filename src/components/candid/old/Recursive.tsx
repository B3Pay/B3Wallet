import { useState, useEffect } from "react"
import FieldRoute, { FieldRouteProps } from "./FieldRoute"
import { ExtractedField } from "@ic-reactor/store/dist/candid"

interface RecursiveProps extends FieldRouteProps {}

const Recursive: React.FC<RecursiveProps> = ({
  methodField: field,
  errors,
  registerName,
  ...rest
}) => {
  const [extractedField, setExtractedFields] = useState<ExtractedField>()

  useEffect(() => {
    const fields = field.extract?.()
    setExtractedFields(fields)
  }, [field])

  return extractedField ? (
    <FieldRoute
      methodField={extractedField}
      registerName={registerName}
      errors={errors?.[field.label as never]}
      {...rest}
    />
  ) : (
    <div>Loading...</div>
  )
}
export default Recursive
