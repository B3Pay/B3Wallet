import { HttpAgent, Identity } from "@dfinity/agent"
import { AuthClient } from "@dfinity/auth-client"
import { B3_PAYMENT_CANISTER_ID } from "helpers/config"
import { b3_payment, createActor } from "../../declarations/b3_payment"

export function getHttpAgent(identity: Identity) {
  console.log("getHttpAgent", process.env.NEXT_PUBLIC_IC_HOST)
  return new HttpAgent({
    host: process.env.NEXT_PUBLIC_IC_HOST,
    identity
  })
}

export async function makeB3PaymentActor() {
  return await AuthClient.create().then(async client => {
    await client?.isAuthenticated()

    return createActor(B3_PAYMENT_CANISTER_ID, {
      agent: getHttpAgent(client.getIdentity())
    })
  })
}

export type B3Payment = typeof b3_payment
