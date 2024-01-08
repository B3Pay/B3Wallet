import { Value } from "@src/declarations/b3system/b3system.did"

export type Metadata = Array<[string, Value]>

export type ExtractedMetadata = {
  logo: Uint8Array
  name: string
  repo: string
  [key: string]: any
}

export const extractMetadata = (metadata: Metadata): ExtractedMetadata => {
  return metadata.reduce((acc, [key, value]) => {
    acc[key] = Object.values(value)[0]
    return acc
  }, {} as ExtractedMetadata)
}
