import { Box } from "components/ui/box"
import FieldRoute, { FieldRouteProps } from "./FieldRoute"
import { Label } from "components/ui/label"

interface RecordProps extends FieldRouteProps {}

const Record: React.FC<RecordProps> = ({
  methodField: field,
  errors,
  registerName
}) => {
  return (
    <Box>
      <Label>{field.label}</Label>
      {field.fields?.map((field, index) => (
        <FieldRoute
          key={index}
          registerName={`${registerName}.${field.label}`}
          methodField={field}
          errors={errors?.[field.label as never]}
        />
      ))}
    </Box>
  )
}

export default Record
