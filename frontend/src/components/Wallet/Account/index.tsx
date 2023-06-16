/* eslint-disable no-unused-vars */
import { Box } from "@chakra-ui/react"

import { Principal } from "@dfinity/principal"
import { WalletAccountView } from "declarations/b3_wallet/b3_wallet.did"
import { ChainNetwork, ChainSymbol } from "helpers/utiles"
import { useMemo } from "react"
import { B3Wallet } from "service/actor"
import Loading from "../../Loading"
import AccountTitle from "./AccountTitle"
import ChainCards, { Addresses } from "./ChainCards"

interface AccountProps extends WalletAccountView {
  actor: B3Wallet
  loading: boolean
  refetchAccount: () => void
}

export interface Balances {
  EVM: bigint
  BTC: bigint
  ICP: bigint
  ICRC: bigint
}

export interface Loadings {
  global: boolean
  EVM: boolean
  BTC: boolean
  ICP: boolean
  ICRC: boolean
}

const Account: React.FC<AccountProps> = ({
  actor,
  id,
  name,
  loading,
  pendings,
  addresses,
  environment,
  refetchAccount
}) => {
  console.log({ pendings })

  const addressesWithChain: Addresses = useMemo(() => {
    const addressMap: Addresses = {}

    addresses.map(([chain, address]) => {
      const symbol = Object.keys(chain)[0] as ChainSymbol
      const network = Object.values(chain)[0] as ChainNetwork

      const networkDetail =
        network === null
          ? null
          : typeof network === "string"
          ? network
          : typeof network === "bigint"
          ? network.toString()
          : (network as Principal)._isPrincipal
          ? network.toString()
          : Object.keys(network)[0]

      const addressItem = {
        address,
        symbol,
        network,
        networkDetail,
        chain
      }

      if (addressMap[symbol]) {
        addressMap[symbol].push(addressItem)
      } else {
        addressMap[symbol] = [addressItem]
      }
    })

    return addressMap
  }, [addresses])

  return (
    <Box position="relative">
      {loading && <Loading />}
      <AccountTitle
        id={id}
        name={name}
        actor={actor}
        environment={environment}
        setLoadings={() => {}}
        refetchAccount={refetchAccount}
      />
      <ChainCards
        addresses={addressesWithChain}
        actor={actor}
        accountId={id}
        refetchAccount={refetchAccount}
      />
    </Box>
  )
}

export default Account
