import { Slot } from "@radix-ui/react-slot"
import { cva, VariantProps } from "class-variance-authority"
import { cn } from "lib/utils"
import {
  BgColorVariant,
  bgColorVariants,
  MarginVariant,
  marginVariants,
  PaddingVariant,
  paddingVariants,
  hoverBgColorVariants,
  HoverBgColorVariant,
  colorVariants,
  ColorVariant,
  HoverColorVariant,
  BorderWidthVariant,
  borderWidthVariants
} from "lib/variants"
import * as React from "react"

const boxVariants = cva("", {
  variants: {
    size: {
      xs: "text-xs",
      sm: "text-sm",
      md: "text-base",
      lg: "text-lg",
      xl: "text-xl"
    },
    hoverable: {
      true: "hover:text-foreground"
    }
  }
})

export interface BoxProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, "color">,
    VariantProps<typeof boxVariants>,
    VariantProps<HoverColorVariant>,
    VariantProps<HoverBgColorVariant>,
    VariantProps<ColorVariant>,
    VariantProps<BgColorVariant>,
    VariantProps<BorderWidthVariant>,
    VariantProps<PaddingVariant>,
    VariantProps<MarginVariant> {
  asChild?: boolean
  hoverable?: boolean
  roundSize?: "xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | "full" | null
}

const Box = React.forwardRef<HTMLDivElement, BoxProps>(
  (
    {
      asChild,
      bgColor = "inherit",
      color,
      hoverColor,
      hoverBgColor,
      hoverable,
      padding,
      paddingBottom,
      paddingLeft,
      paddingRight,
      paddingTop,
      margin,
      marginBottom,
      marginLeft,
      marginRight,
      marginTop,
      roundSize,
      borderWidth,
      size,
      children,
      className,
      ...props
    },
    ref
  ) => {
    const Comp = asChild ? Slot : "div"

    return (
      <Comp
        ref={ref}
        className={cn(
          boxVariants({
            size,
            hoverable
          }),
          colorVariants({ color }),
          hoverBgColorVariants(50)({ hoverBgColor }),
          bgColorVariants(5)({ bgColor }),
          paddingVariants({
            padding,
            paddingBottom,
            paddingLeft,
            paddingRight,
            paddingTop
          }),
          marginVariants({
            margin,
            marginBottom,
            marginLeft,
            marginRight,
            marginTop
          }),
          borderWidthVariants({ borderWidth }),
          roundSize && `rounded-${roundSize}`,
          className
        )}
        {...props}
      >
        {children}
      </Comp>
    )
  }
)

Box.displayName = "Box"

export { Box }
