import FieldRoute, { FieldRouteProps } from "./FieldRoute"

interface TupleProps extends FieldRouteProps {}

const Tuple: React.FC<TupleProps> = ({
  methodField: field,
  registerName,
  errors
}) => {
  return (
    <div className="w-full">
      {/* <div className="font-semibold">{field.label.toTitleCase()}</div> */}
      {field.fields?.map((field, index) => (
        <FieldRoute
          key={index}
          registerName={`${registerName}.[${index}]`}
          errors={errors?.[index as never]}
          methodField={field}
        />
      ))}
    </div>
  )
}

export default Tuple
