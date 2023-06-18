export const getHostFromUrl = (hostUrl: string) => {
  try {
    const url = new URL(hostUrl)
    return url.host
  } catch (error) {
    return ""
  }
}

export function convertBigIntToNumber(bigintValue: BigInt): number | null {
  const numberValue = Number(bigintValue)
  if (Number.isSafeInteger(numberValue)) {
    return numberValue
  } else {
    console.warn(
      "The BigInt value is too large to be safely converted to Number."
    )
    return null // or throw an error, or handle in some other way
  }
}

export interface PendingTranscation {
  currentConfirmations: number | null
  requiredConfirmations: number | null
}

export function extractConfirmations(msg: string): PendingTranscation {
  let currentConfirmationsMatch = msg.match(/Current confirmations: (\d+)/)
  let requiredConfirmationsMatch = msg.match(/required confirmations: (\d+)/)

  let result = {
    currentConfirmations: currentConfirmationsMatch
      ? parseInt(currentConfirmationsMatch[1])
      : null,
    requiredConfirmations: requiredConfirmationsMatch
      ? parseInt(requiredConfirmationsMatch[1])
      : null
  }

  return result
}

export const compileError = (description: string[], title: React.ReactNode) => {
  if (description.length > 1) {
    return {
      title: description[1],
      description: description[2]
    }
  } else {
    return {
      title,
      description: description[0]
    }
  }
}
