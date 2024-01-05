import B3System from "./B3System"
import Head from "next/head"

function B3SystemPage() {
  return (
    <div>
      <Head>
        <title>B3Wallet</title>
      </Head>
      {/* <CreateApp />
        <CreateWallet />
        <UserStatus />
        <AppCanisterStatus />
        <AppCanisterVersion /> */}
      <B3System />
    </div>
  )
}

export default B3SystemPage
