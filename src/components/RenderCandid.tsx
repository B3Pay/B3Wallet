import { IDL } from "@dfinity/candid"
import { Type } from "@dfinity/candid/lib/cjs/idl"
import { useState } from "react"
import VarianComponent from "./candid/VarianComponent"
import { Button } from "./ui/button"
import { Input } from "./ui/input"

interface RenderCandidProps {
  argTypes: Type[]
  buttonName?: string
  loading?: boolean
  clickHandler?: () => void
}

export interface TypeWithFields extends Type {
  _fields: Array<[string, Type]>
}

export interface TypeWithComponent extends Type {
  _components: Array<[TypeWithFields, TypeWithFields]>
}

type RenderInputProps = {
  type: TypeWithFields
  name: string
  value: any
  handleChange: (key: string, value: any) => void
}

const renderInput = ({ type, name, value, handleChange }: RenderInputProps) => {
  switch (type.constructor) {
    case IDL.IntClass:
      return (
        <Input
          type="number"
          value={value}
          onChange={e => handleChange(name, e.target.value)}
        />
      )
    case IDL.NatClass:
      return (
        <Input
          type="number"
          value={value}
          onChange={e => handleChange(name, e.target.value)}
        />
      )
    case IDL.FixedNatClass:
      return (
        <Input
          type="number"
          value={value}
          onChange={e => handleChange(name, e.target.value)}
        />
      )
    case IDL.TextClass:
      return (
        <Input
          type="text"
          value={value}
          onChange={e => handleChange(name, e.target.value)}
        />
      )
    case IDL.VecClass:
      // Example: VecClass might be rendered as a select if that makes sense for your data
      console.log(type._type)
      return renderInput({
        type: (type as IDL.VecClass<any>)._type._fields[1][1]._type,
        name,
        value,
        handleChange
      })
    case IDL.VariantClass:
      // Example: VecClass might be rendered as a select if that makes sense for your data
      console.log(type as IDL.VecClass<any>)
      return (
        <VarianComponent
          type={type as TypeWithFields}
          name={name}
          value={value}
          handleChange={handleChange}
        />
      )

    // Add more cases for different types
    default:
      return <div>Unsupported type: {type.name}</div>
  }
}

const RenderCandid: React.FC<RenderCandidProps> = ({
  argTypes,
  clickHandler,
  loading,
  buttonName
}) => {
  const [formData, setFormData] = useState<any>({})

  const handleChange = (key: string, value: any) => {
    setFormData({ ...formData, [key]: value })
  }

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    // Handle form submission, maybe send formData to a server or process it
    console.log(formData)
  }

  return (
    <form onSubmit={handleSubmit}>
      {(argTypes as TypeWithFields[]).map(({ _fields }) => {
        return _fields.map(([name, type]) => (
          <div key={name}>
            <label htmlFor={name}>{name}</label>
            {renderInput({ type, name, value: formData[name], handleChange })}
          </div>
        ))
      })}
      <Button isLoading={loading} onClick={clickHandler}>
        {buttonName}
      </Button>
    </form>
  )
}

export default RenderCandid
