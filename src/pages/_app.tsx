import "styles/global.css"

interface AppProps {
  Component: React.FC
  pageProps: any
}

const App: React.FC<AppProps> = ({ Component, pageProps }) => {
  return <Component {...pageProps} />
}

export default App
