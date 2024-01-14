import React from "react"
import { Tuple, TupleProps } from "./tuple"
import { Vector, VectorProps } from "./vector"
import { Record, RecordProps } from "./record"
import { Variant, VariantProps } from "./variant"
import { Optional, OptionalProps } from "./optional"
import { Recursive, RecursiveProps } from "./recursive"
import { FieldError, FieldErrorsImpl, Merge } from "react-hook-form"
import {
  DynamicFieldType,
  ExtractedFieldType
} from "@ic-reactor/react/dist/types"

import { Principal, PrincipalProps } from "../inputs/principal"
import { Boolean, BooleanProps } from "../inputs/boolean"
import { Number, NumberProps } from "../inputs/number"
import { Null, NullProps } from "../inputs/null"
import { Text, TextProps } from "../inputs/text"

export interface RouteProps<T extends ExtractedFieldType = any> {
  extractedField: DynamicFieldType<T>
  registerName: string
  shouldUnregister?: boolean
  errors: FieldError | Merge<FieldError, FieldErrorsImpl<any>> | undefined
}

const Route: React.FC<RouteProps> = props => {
  switch (props.extractedField.type) {
    case "vector":
      return <Vector {...(props as VectorProps)} />
    case "optional":
      return <Optional {...(props as OptionalProps)} />
    case "record":
      return <Record {...(props as RecordProps)} />
    case "tuple":
      return <Tuple {...(props as TupleProps)} />
    case "variant":
      return <Variant {...(props as VariantProps)} />
    case "recursive":
      return <Recursive {...(props as RecursiveProps)} />
    case "principal":
      return <Principal {...(props as PrincipalProps)} />
    case "null":
      return <Null {...(props as NullProps)} />
    case "boolean":
      return <Boolean {...(props as BooleanProps)} />
    case "number":
      return <Number {...(props as NumberProps)} />
    default:
      return <Text {...(props as TextProps)} />
  }
}

export { Route }
