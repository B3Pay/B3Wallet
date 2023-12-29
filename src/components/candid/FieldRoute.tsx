import React from "react"
import Vector from "./Vector"
import Input from "./FieldInput"
import Optional from "./Optional"
import Variant from "./Variant"
import Recursive from "./Recursive"
import Record from "./Record"
import Tuple from "./Tuple"
import { FieldError, FieldErrorsImpl, Merge } from "react-hook-form"
import Principal from "./Principal"
import { ExtractedField } from "@ic-reactor/store/dist/candid"

export interface FieldRouteProps {
  methodField: ExtractedField
  registerName: string
  errors: FieldError | Merge<FieldError, FieldErrorsImpl<any>> | undefined
}

const FieldRoute: React.FC<FieldRouteProps> = props => {
  switch (props.methodField.type) {
    case "vector":
      return <Vector {...props} />
    case "optional":
      return <Optional {...props} />
    case "record":
      return <Record {...props} />
    case "tuple":
      return <Tuple {...props} />
    case "variant":
      return <Variant {...props} />
    case "recursive":
      return <Recursive {...props} />
    case "principal":
      return <Principal {...props} />
    case "null":
      return null
    default:
      return <Input {...props} />
  }
}

export default FieldRoute
