import HeadTitle from "@src/components/HeadTitle"
import B3System from "./B3System"
import { useSystemAuthClient } from "@src/service/system"
import { Card } from "@src/components/ui/card"
import Image from "next/image"

function B3SystemPage() {
  const { identity } = useSystemAuthClient()

  return (
    <div>
      <HeadTitle title="B3System" />
      <Card
        title={
          <div className="flex flex-row justify-between items-center w-full">
            <h3 className="text-xl font-semibold">B3System</h3>
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
            src="assets/store-logo.png"
            width={35}
            height={35}
            alt="b3wallet"
          />
        }
      />
      <B3System />
    </div>
  )
}

export default B3SystemPage
