import Image, { ImageProps } from "next/image"
import React, { useState, useEffect } from "react"

interface ImageFromBlobProps extends Omit<ImageProps, "src"> {
  imageData: Uint8Array
  name: string
  imageType?: string
}

const ImageFromBlob: React.FC<ImageFromBlobProps> = ({
  imageData,
  imageType = "image/png",
  ...props
}) => {
  const [imageUrl, setImageUrl] = useState<string>()

  useEffect(() => {
    // Convert Uint8Array to Blob
    const blob = new Blob([imageData], { type: imageType })

    // Create a URL for the Blob
    const url = URL.createObjectURL(blob)
    setImageUrl(url)

    // Clean up
    return () => {
      URL.revokeObjectURL(url)
    }
  }, [imageData, imageType])

  // eslint-disable-next-line jsx-a11y/alt-text
  return imageUrl && <Image {...props} src={imageUrl} />
}

export default ImageFromBlob
