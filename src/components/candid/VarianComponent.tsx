import { TypeWithFields } from "components/RenderCandid"
import { Select, SelectContent, SelectItem, SelectTrigger } from "../ui/select"

interface VarianComponentProps {
  type: TypeWithFields
  name: string
  value: any
  handleChange: (key: string, value: any) => void
}

const VarianComponent: React.FC<VarianComponentProps> = ({
  type,
  name,
  value,
  handleChange
}) => {
  return (
    <Select>
      <SelectTrigger>{name}</SelectTrigger>
      <SelectContent>
        {type._fields.map(([name, type]) => (
          <SelectItem
            value={value}
            key={name}
            onClick={() => handleChange(name, type)}
          >
            {name}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  )
}

export default VarianComponent
