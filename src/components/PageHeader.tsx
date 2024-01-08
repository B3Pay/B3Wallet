import { Card, CardProps } from "@src/components/ui/card"
import Image from "next/image"
import Head from "next/head"
import { useAgent } from "@ic-reactor/react"

interface PageHeaderProps extends CardProps {
  title: string
}

const PageHeader: React.FC<PageHeaderProps> = ({ title, ...rest }) => {
  const { useAuthClient } = useAgent()
  const { identity } = useAuthClient()

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
              {identity?.getPrincipal().toText()}
            </span>
          </div>
        }
        iconProps={{
          size: "xl",
          roundSide: "l",
          diagonalRoundSide: "none",
          color: "muted"
        }}
        icon={
          <Image
            src={`${
              window.location.origin
            }/assets/logo/${title.toLowerCase()}.png`}
            onError={e => (e.currentTarget.src = "assets/logo/b3.png")}
            width={35}
            height={35}
            alt="b3wallet"
          />
        }
        {...rest}
      />
    </div>
  )
}
export default PageHeader
