import FieldRoute, { FieldRouteProps } from "./FieldRoute"

interface TupleProps extends FieldRouteProps {}

const Tuple: React.FC<TupleProps> = ({ methodField, registerName, errors }) => {
  return methodField.fields.map((field, index) => (
    <FieldRoute
      key={index}
      registerName={`${registerName}.[${index}]`}
      errors={errors?.[index as never]}
      methodField={field}
    />
  ))
}

export default Tuple
