import { toHex } from "@dfinity/candid/lib/cjs"
import { clsx, type ClassValue } from "clsx"
import { CanisterStatusResponse } from "declarations/b3_system/b3_system.did"
import { twMerge } from "tailwind-merge"

export const errorHandler = (error: Error | unknown | null): string => {
  // Check if an error was passed
  if (!error) {
    // Return an empty string if no error was passed
    return ""
  }

  const errorString = error.toString()

  // Regular expression to match the specific error text
  // Error: Call was rejected:
  // Request ID: c2eb7e506a48634d1f839095cea252f0b99171807b576a4518b26260bb9dbd53
  // Reject code: 5
  // Reject text: Canister bkyz2-fmaaa-aaaaa-qaaaq-cai trapped explicitly: User already exists!
  const regex = /Reject text: (.*)/

  // Use the regular expression to find the error message
  const match = errorString.match(regex)

  // Check if a match was found
  if (match && match[1]) {
    // Return the extracted error message
    return match[1]
  } else {
    // Return a generic error message if no specific message was found
    return "An unknown error occurred."
  }
}

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
    return toHex(moduleHash[0] as Uint8Array)
  }
  return undefined
}

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export const focusRing =
  "focusable-element focus:outline-none ring-inset focus:ring-1 focus:ring-foreground focus:ring-offset-1"
