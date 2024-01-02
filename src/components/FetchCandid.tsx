import { useEffect } from "react"
import { getRemoteDidJs } from "service/candid"

interface FetchCandidProps {}

const FetchCandid: React.FC<FetchCandidProps> = ({}) => {
  useEffect(() => {
    // fetch candid
    getRemoteDidJs("2227b-baaaa-aaaao-abd6a-cai")
  }, [])

  return <div></div>
}

export default FetchCandid
