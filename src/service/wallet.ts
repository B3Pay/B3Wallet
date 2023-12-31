import { createReActor } from "@ic-reactor/react"
import { ReActorMethodField } from "@ic-reactor/store"
import { b3_wallet, canisterId, idlFactory } from "declarations/b3_wallet"

export type B3Wallet = typeof b3_wallet

export const {
  useQueryCall: useWalletQuery,
  useUpdateCall: useWalletUpdate,
  useAuthClient: useWalletAuthClient,
  useActorStore: useWalletActorStore,
  useMethodFields: useWalletMethodFields
} = createReActor<B3Wallet>({
  canisterId,
  idlFactory,
  withDevtools: true
})

export type WalletDynamicField = ReActorMethodField<B3Wallet>
