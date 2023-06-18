import { Box, Button, Heading, Image, Stack, Text } from "@chakra-ui/react"

function Store({ buyHandler }) {
  const products = [
    { id: 1, name: "Product 2", price: 10_00000000, network: "ICP" },
    { id: 2, name: "Product 1", price: 100, network: "CKBTC" },
    { id: 3, name: "Product 3", price: 100, network: "BTC" },
    { id: 4, name: "Product 3", price: 100, network: "ETH" }
  ]

  return (
    <Box p={5}>
      <Heading mb={5}>Basic Shop</Heading>
      <Stack direction="row" spacing={1} mb={5} wrap="wrap">
        {products.map(product => (
          <Box
            key={product.id}
            borderWidth="1px"
            borderRadius="lg"
            overflow="hidden"
            mb={5}
          >
            <Box>
              <Image
                src="https://via.placeholder.com/300x150"
                alt={product.name}
              />
            </Box>
            <Box p="6">
              <Heading size="md" mb={2}>
                {product.name}
              </Heading>
              <Text mb={2}>
                {(product.price / 10 ** 8).toString()} {product.network}
              </Text>
              <Button colorScheme="teal" onClick={() => buyHandler(product)}>
                Buy Now
              </Button>
            </Box>
          </Box>
        ))}
      </Stack>
    </Box>
  )
}

export default Store
