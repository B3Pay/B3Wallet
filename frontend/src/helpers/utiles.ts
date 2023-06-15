import { Principal } from "@dfinity/principal"
import {
  BtcNetwork,
  ChainEnum
} from "../../declarations/b3_wallet/b3_wallet.did"

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

export interface ChainTypeMap {
  BTC: BtcNetwork
  EVM: bigint
  ICP: null
  CKBTC: null
  ICRC: Principal
}

export type ChainSymbol = keyof ChainTypeMap

export type ChainNetwork = ChainTypeMap[ChainSymbol]

export interface ChainTypeString {
  BTC: "Mainnet" | "Testnet" | "Regtest"
  EVM: string
  ICP: null
  CKBTC: null
  ICRC: string
}

export type ChainTypeStringNetwork = ChainTypeString[ChainSymbol]

export function handleChainType<T extends ChainSymbol>(
  network: ChainTypeString[T],
  symbol: T
): ChainEnum {
  switch (symbol) {
    case "BTC":
      const btcNetwork = { [network]: null } as BtcNetwork

      return { BTC: btcNetwork }
    case "CKBTC": {
      const btcNetwork = { [network]: null } as BtcNetwork

      return { CKBTC: btcNetwork }
    }

    case "EVM": {
      const evmChainId = BigInt(network)

      return { EVM: evmChainId }
    }
    case "ICP": {
      return { ICP: null }
    }

    case "ICRC": {
      const icrcId = Principal.fromText(network)

      return { ICRC: icrcId }
    }
    default: {
      throw new Error("Invalid ChainType")
    }
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

export const compileError = (description: string[]) => {
  if (description.length > 1) {
    return {
      title: description[1],
      description: description[2]
    }
  } else {
    return {
      title: "Error",
      description: description[0]
    }
  }
}
