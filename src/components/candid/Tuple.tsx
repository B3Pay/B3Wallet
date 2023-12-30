import FieldRoute, { FieldRouteProps } from "./FieldRoute"

interface TupleProps extends FieldRouteProps {}

const Tuple: React.FC<TupleProps> = ({
  methodField: field,
  registerName,
  errors
}) => {
  return field.fields?.map((field, index) => (
    <FieldRoute
      key={index}
      registerName={`${registerName}.[${index}]`}
      errors={errors?.[index as never]}
      methodField={field}
    />
  ))
}

export default Tuple
