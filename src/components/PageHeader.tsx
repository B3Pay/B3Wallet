import { useWalletAuthClient } from "@src/service/wallet"
import { Card } from "@src/components/ui/card"
import Image from "next/image"
import { PropsWithChildren } from "react"
import Head from "next/head"

interface PageHeaderProps extends PropsWithChildren {
  title: string
}

const PageHeader: React.FC<PageHeaderProps> = ({ children, title }) => {
  const { identity } = useWalletAuthClient()
  return (
    <div>
      <Head>
        <title>{title}</title>
      </Head>
      <Card
        title={
          <div className="flex flex-row justify-between items-center w-full">
            <h3 className="text-xl font-semibold">{title}</h3>
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
      {children}
    </div>
  )
}
export default PageHeader
