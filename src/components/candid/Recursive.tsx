import { useState, useEffect } from "react"
import FieldSwitch, { FieldSwitchProps } from "./FieldSwitch"
import { ExtractedField } from "@ic-reactor/store/dist/candid"

interface RecursiveProps extends FieldSwitchProps {}

const Recursive: React.FC<RecursiveProps> = ({
  methodField: field,
  errors,
  registerName
}) => {
  const [extractedField, setExtractedFields] = useState<ExtractedField>()

  useEffect(() => {
    const fields = field.extract?.()
    setExtractedFields(fields)
  }, [field])

  return extractedField ? (
    <FieldSwitch
      methodField={extractedField}
      registerName={registerName}
      errors={errors?.[field.label as never]}
    />
  ) : (
    <div>Loading...</div>
  )
}
export default Recursive
