import { createAgentManager } from "@ic-reactor/react"

export const agentManager = createAgentManager({
  host: process.env.NEXT_PUBLIC_IC_HOST
})
