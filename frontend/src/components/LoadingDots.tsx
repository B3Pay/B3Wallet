import { useEffect, useState } from "react"

interface LoadingDotsProps {
  title?: string
}

const LoadingDots: React.FC<LoadingDotsProps> = ({ title }) => {
  const [dots, setDots] = useState(".")

  useEffect(() => {
    const timer = setInterval(() => {
      setDots(dots => (dots.length < 3 ? dots + "." : "."))
    }, 500)

    return () => clearInterval(timer)
  }, [])

  return <>{(title ? title : "Loading") + dots}</>
}

export default LoadingDots
