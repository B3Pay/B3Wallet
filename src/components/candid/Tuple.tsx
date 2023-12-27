import FieldSwitch, { FieldSwitchProps } from "./FieldSwitch"

interface TupleProps extends FieldSwitchProps {}

const Tuple: React.FC<TupleProps> = ({
  methodField: field,
  registerName,
  errors
}) => {
  return (
    <div className="w-full">
      <div className="font-semibold">{field.label}</div>
      {field.fields?.map((field, index) => (
        <FieldSwitch
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
