import FormFieldSwitch, { FormFieldSwitchProps } from "./FieldSwitch"

interface TupleProps extends FormFieldSwitchProps {}

const Tuple: React.FC<TupleProps> = ({ field, registerName, errors }) => {
  return (
    <div className="w-full">
      <div className="font-semibold">{field.label}</div>
      {field.fields?.map((field, index) => (
        <FormFieldSwitch
          key={index}
          registerName={`${registerName}.[${index}]`}
          errors={errors?.[index as never]}
          field={field}
        />
      ))}
    </div>
  )
}

export default Tuple
