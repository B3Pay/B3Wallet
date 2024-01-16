import React from "react"
import { FieldError, FieldErrorsImpl, Merge } from "react-hook-form"
import { DynamicFieldType, ExtractedFieldType } from "@ic-reactor/store"

import { Tuple, TupleProps } from "./tuple"
import { Vector, VectorProps } from "./vector"
import { Record, RecordProps } from "./record"
import { Variant, VariantProps } from "./variant"
import { Optional, OptionalProps } from "./optional"
import { Recursive, RecursiveProps } from "./recursive"

import { Principal, PrincipalProps } from "../inputs/principal"
import { Boolean, BooleanProps } from "../inputs/boolean"
import { Number, NumberProps } from "../inputs/number"
import { Null, NullProps } from "../inputs/null"
import { Text, TextProps } from "../inputs/text"

export interface CandidRouteProps<T extends ExtractedFieldType = any> {
  extractedField: DynamicFieldType<T>
  registerName: string
  shouldUnregister?: boolean
  errors: FieldError | Merge<FieldError, FieldErrorsImpl<any>> | undefined
}

const CandidRoute: React.FC<CandidRouteProps> = props => {
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
    case "number":
      return <Number {...(props as NumberProps)} />
    case "boolean":
      return <Boolean {...(props as BooleanProps)} />
    case "null":
      return <Null {...(props as NullProps)} />
    default:
      return <Text {...(props as TextProps)} />
  }
}

export { CandidRoute }
