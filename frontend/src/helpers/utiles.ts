import { Principal } from "@dfinity/principal"
import {
  BtcNetwork,
  ChainType
} from "../../declarations/b3_wallet/b3_wallet.did"

export const getHostFromUrl = (hostUrl: string) => {
  try {
    const url = new URL(hostUrl)
    return url.host
  } catch (error) {
    return ""
  }
}

export interface ChainTypeMap {
  BTC: BtcNetwork
  EVM: bigint
  ICP: null
  ICRC: Principal
}

export type ChainSymbol = keyof ChainTypeMap

export type ChainNetwork = ChainTypeMap[ChainSymbol]

export interface ChainTypeString {
  BTC: "Mainnet" | "Testnet" | "Regtest"
  EVM: string
  ICP: null
  ICRC: string
}

export type ChainTypeStringNetwork = ChainTypeString[ChainSymbol]

export function handleChainType<T extends ChainSymbol>(
  network: ChainTypeString[T],
  symbol: T
): ChainType {
  switch (symbol) {
    case "BTC": {
      const btcNetwork = { [network]: null } as BtcNetwork

      return { BTC: btcNetwork }
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
