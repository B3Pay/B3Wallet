import { Box } from "@chakra-ui/react"
import { useRouter } from "next/router"

interface paymentProps {}

const payment: React.FC<paymentProps> = ({}) => {
  let { query } = useRouter()
  console.log(query)
  return (
    <Box mt={10}>
      <h1>Payment</h1>
    </Box>
  )
}

export default payment
