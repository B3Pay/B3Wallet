import { useState, useRef, useCallback, useLayoutEffect } from "react"

export const useFlipResize = () => {
  const [isFlipped, setIsFlipped] = useState(false)
  const containerRef = useRef<HTMLDivElement>(null)
  const backRef = useRef<HTMLDivElement>(null)
  const frontRef = useRef<HTMLDivElement>(null)

  const updateContainerHeight = useCallback(() => {
    if (containerRef.current && frontRef.current && backRef.current) {
      const currentHeight = isFlipped
        ? backRef.current.clientHeight
        : frontRef.current.clientHeight

      containerRef.current.style.height = `${currentHeight}px`
    }
  }, [isFlipped])

  const flip = () => {
    setIsFlipped(prev => !prev)
  }

  useLayoutEffect(() => {
    updateContainerHeight()
  }, [updateContainerHeight])

  return { isFlipped, flip, backRef, frontRef, containerRef }
}

export const useResize = () => {
  const [toggle, setToggle] = useState(false)
  const containerRef = useRef<HTMLDivElement>(null)
  const childRef = useRef<HTMLDivElement>(null)

  const updateContainerHeight = useCallback(() => {
    if (containerRef.current && childRef.current) {
      const currentHeight = childRef.current.clientHeight
      containerRef.current.style.height = `${currentHeight}px`
    }
  }, [])

  const toggleResize = () => {
    setToggle(prev => !prev)
    updateContainerHeight()
  }

  useLayoutEffect(() => {
    updateContainerHeight()
  }, [updateContainerHeight])

  return { toggle, toggleResize, childRef, containerRef }
}
