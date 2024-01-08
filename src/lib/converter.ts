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

export const truncateString = (hash: string, length: number = 4) => {
  return hash.slice(0, length) + "..." + hash.slice(-length)
}

export const nanoTimeToDate = (nanoTime: bigint) => {
  return new Date(Number(nanoTime) / 1000000)
}

export const sizeToMbWithUnit = (size: bigint) => {
  const sizeInMb = Number(size) / 1000000
  return sizeInMb.toFixed(2)
}
