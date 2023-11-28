import { toHexString } from "@dfinity/candid"
import { clsx, type ClassValue } from "clsx"
import { CanisterStatusResponse } from "declarations/b3_system/b3_system.did"
import { twMerge } from "tailwind-merge"

export const objectToString = (data: any) => {
  return JSON.stringify(
    data,
    (_, value) => (typeof value === "bigint" ? value.toString() : value),
    2
  )
}

export const getModuleHash = (
  status: CanisterStatusResponse
): string | undefined => {
  const moduleHash = status.module_hash

  if (moduleHash.length === 1) {
    return toHexString(moduleHash[0] as Uint8Array)
  }
  return undefined
}

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export const focusRing =
  "focusable-element focus:outline-none ring-inset focus:ring-1 focus:ring-foreground"
