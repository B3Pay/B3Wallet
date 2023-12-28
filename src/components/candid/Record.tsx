import FieldRoute, { FieldRouteProps } from "./FieldRoute"

interface RecordProps extends FieldRouteProps {}

const Record: React.FC<RecordProps> = ({
  methodField: field,
  errors,
  registerName
}) => {
  return (
    <div className="w-full">
      <div className="font-semibold">{field.label}</div>
      {field.fields?.map((field, index) => (
        <FieldRoute
          key={index}
          registerName={`${registerName}.${field.label}`}
          methodField={field}
          errors={errors?.[field.label as never]}
        />
      ))}
    </div>
  )
}

export default Record
