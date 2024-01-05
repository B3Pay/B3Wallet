import { useWalletAuthClient } from "@src/service/wallet"
import B3Wallet from "./B3Wallet"
import { Card } from "@src/components/ui/card"
import Image from "next/image"
import HeadTitle from "@src/components/HeadTitle"

function B3WalletPage() {
  const { identity } = useWalletAuthClient()
  return (
    <div>
      <HeadTitle title="B3Wallet" />
      <Card
        title={
          <div className="flex flex-row justify-between items-center w-full">
            <h3 className="text-xl font-semibold">B3Wallet</h3>
            <span className="text-sm px-4">
              {identity?.getPrincipal().toString()}
            </span>
          </div>
        }
        marginBottom="sm"
        iconProps={{
          size: "xl",
          roundSide: "l",
          color: "muted"
        }}
        icon={
          <Image
            src="assets/wallet-logo.png"
            width={35}
            height={35}
            alt="b3wallet"
          />
        }
      />
      <B3Wallet />
    </div>
  )
}

export default B3WalletPage
