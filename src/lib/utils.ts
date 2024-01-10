import { toHexString } from "@dfinity/candid/lib/cjs"
import { clsx, type ClassValue } from "clsx"
import { CanisterStatusResponse } from "@src/declarations/b3system/b3system.did"
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
    const regex2 = /"Message": (.*)/

    const match2 = errorString.match(regex2)

    if (match2 && match2[1]) {
      return match2[1]
    }

    // Return a generic error message if no specific message was found
    return "An unknown error occurred."
  }
}

export const objectToString = (data: any) => {
  return JSON.stringify(
    data,
    (name, value) => {
      if (name === "Blob") {
        return value.slice(0, 100) + "..."
      }
      switch (typeof value) {
        case "bigint":
          return value.toString()
        default:
          return value
      }
    },
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
  "focusable-element focus:outline-none ring-inset focus:ring-1 focus:ring-foreground focus:ring-offset-1"

String.prototype.toTitleCase = function () {
  return this.split("_")
    .map(fragment =>
      fragment
        .replace(/([a-z])([A-Z])/g, "$1 $2") // Split camelCase
        .split(" ")
        .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
        .join(" ")
    )
    .join(" ")
}
