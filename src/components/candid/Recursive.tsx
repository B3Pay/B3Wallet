import { useState, useEffect } from "react"
import FormFieldSwitch, { FormFieldSwitchProps } from "./FieldSwitch"
import { ExtractedField } from "@ic-reactor/store/dist/candid"

interface RecursiveProps extends FormFieldSwitchProps {}

const Recursive: React.FC<RecursiveProps> = ({
  field,
  errors,
  registerName
}) => {
  const [extractedField, setExtractedFields] = useState<ExtractedField>()

  useEffect(() => {
    const fields = field.extract?.()
    setExtractedFields(fields)
  }, [field])

  return extractedField ? (
    <FormFieldSwitch
      field={extractedField}
      registerName={registerName}
      errors={errors?.[field.label as never]}
    />
  ) : (
    <div>Loading...</div>
  )
}
export default Recursive
