import { Box } from "@src/components/ui/box"
import FieldRoute, { FieldRouteProps } from "./FieldRoute"

interface RecordProps extends FieldRouteProps {}

const Record: React.FC<RecordProps> = ({
  methodField,
  errors,
  registerName,
  ...rest
}) => {
  return (
    <Box>
      {/* <Label>{field.label}</Label> */}
      {methodField.fields.map((field, index) => (
        <FieldRoute
          key={index}
          registerName={`${registerName}.${field.label}`}
          methodField={field}
          errors={errors?.[field.label as never]}
          {...rest}
        />
      ))}
    </Box>
  )
}

export default Record
