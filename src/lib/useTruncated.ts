import { useState, useEffect, useMemo } from "react"

const useTruncated = (
  address: string,
  size: "xs" | "sm" | "md" | "lg" | "xl" | null
) => {
  const [isLargerThan500, setIsLargerThan500] = useState(
    window.innerWidth > 568
  )

  useEffect(() => {
    const handleResize = () => {
      setIsLargerThan500(window.innerWidth > 568)
    }

    window.addEventListener("resize", handleResize)

    return () => {
      window.removeEventListener("resize", handleResize)
    }
  }, [])

  const truncatedAddress = useMemo(() => {
    if (address.length <= 20 || (isLargerThan500 && address.length <= 42)) {
      return address
    }

    const maxLength = size === "xs" ? 8 : size === "sm" ? 12 : 20

    const Start = address.slice(0, isLargerThan500 ? maxLength : 8)
    const End = address.slice(isLargerThan500 ? -maxLength : -8)

    return `${Start}...${End}`
  }, [address, isLargerThan500, size])

  return truncatedAddress
}

export default useTruncated
