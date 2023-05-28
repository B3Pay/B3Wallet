import { ChakraProvider } from "@chakra-ui/react"
import { Rubik } from "next/font/google"

const rubik = Rubik({ subsets: ["latin"] })

// 1. Import the extendTheme function
import { extendTheme } from "@chakra-ui/react"

// 2. Extend the theme to include custom colors, fonts, etc
const colors = {
  brand: {
    900: "#1a365d",
    800: "#153e75",
    700: "#2a69ac"
  }
}

const fonts = {
  heading: "var(--font-rubik)",
  body: "var(--font-rubik)"
}

export const theme = extendTheme({ colors, fonts })

interface AppProps {
  Component: React.FC
  pageProps: any
}

const App: React.FC<AppProps> = ({ Component, pageProps }) => {
  return (
    <>
      <style jsx global>
        {`
          :root {
            --font-rubik: ${rubik.style.fontFamily};
          }
        `}
      </style>
      <ChakraProvider theme={theme}>
        <Component {...pageProps} />
      </ChakraProvider>
    </>
  )
}

export default App
