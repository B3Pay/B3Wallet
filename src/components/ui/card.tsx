import * as React from "react"
import { VariantProps, cva } from "class-variance-authority"
import { cn } from "@src/lib/utils"
import {
  BgColorVariant,
  BgGradientVariant,
  MarginVariant,
  PaddingVariant,
  bgColorVariants,
  bgGradientVariants,
  marginVariants,
  paddingVariants
} from "@src/lib/variants"
import { Icon, IconProps } from "./icon"
import { Box, BoxProps } from "./box"

const cardVariants = cva("shadow", {
  variants: {
    size: {
      xs: "text-xs",
      sm: "text-sm",
      md: "text-base",
      lg: "text-lg",
      xl: "text-xl"
    },
    noShadow: {
      true: "shadow-none"
    }
  },
  defaultVariants: {
    size: "md"
  }
})

export interface CardProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, "color" | "title">,
    VariantProps<typeof cardVariants>,
    VariantProps<BgColorVariant>,
    VariantProps<BgGradientVariant>,
    VariantProps<PaddingVariant>,
    VariantProps<MarginVariant> {
  title?: React.ReactNode
  titleProps?: BoxProps
  dashedBorder?: boolean
  icon?: React.ReactNode
  asChild?: boolean
  noRadius?: boolean
  noShadow?: boolean
  border?: 0 | 1 | 2 | 3 | 4
  action?: React.ReactNode
  iconProps?: IconProps
  roundSize?: "xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | null
  roundSide?: "t" | "b" | "l" | "r" | "tl" | "tr" | "bl" | "br" | "none" | null
}

const Card = React.forwardRef<HTMLDivElement, CardProps>(
  (
    {
      className,
      size,
      noShadow,
      noRadius,
      bgColor = "card",
      bgGradient,
      border = 0,
      dashedBorder,
      iconProps,
      titleProps,
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
      icon,
      title,
      roundSize = "xl",
      roundSide,
      action,
      children,
      ...props
    },
    ref
  ) => {
    border = size === "xl" ? 3 : border

    const roundingClass = roundSide
      ? `rounded-${roundSide}-${roundSize}`
      : `rounded-${roundSize}`

    return (
      <div
        ref={ref}
        className={cn(
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
          cardVariants({ size, noShadow }),
          bgColorVariants(50)({ bgColor }),
          bgGradientVariants({ bgGradient }),
          roundingClass,
          className
        )}
      >
        {(title || action || icon) && (
          <div className="flex justify-between w-full items-stretch">
            {icon && (
              <Icon
                diagonalRoundSide="l"
                {...iconProps}
                className={cn(
                  "flex-none flex items-center justify-center",
                  `border-${border}`,
                  dashedBorder ? "border-dashed" : "shadow-button-inner"
                )}
              >
                {icon}
              </Icon>
            )}
            <Box
              bgColor={bgColor}
              size={size}
              className={cn(
                "flex-1 pl-2 flex items-center font-semibold leading-none tracking-tight",
                dashedBorder && "border-dashed",
                `border-t-${border}`
              )}
              {...titleProps}
            >
              {title}
            </Box>
            {action}
          </div>
        )}
        <div
          className={cn(
            noRadius ? "rounded-none" : "rounded-b-lg",
            `border-${border}`,
            "border-t-0",
            dashedBorder && "border-dashed"
          )}
          {...props}
        >
          {children}
        </div>
      </div>
    )
  }
)
Card.displayName = "Card"

const CardHeader = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("flex flex-col space-y-1.5 p-4", className)}
    {...props}
  />
))
CardHeader.displayName = "CardHeader"

export interface CardActionProps extends React.HTMLAttributes<HTMLDivElement> {
  icon?: React.ReactNode
}

const CardTitle = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLHeadingElement>
>(({ className, ...props }, ref) => (
  <h3
    ref={ref}
    className={cn("font-semibold leading-none tracking-tight", className)}
    {...props}
  />
))
CardTitle.displayName = "CardTitle"

const CardDescription = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLParagraphElement>
>(({ className, ...props }, ref) => (
  <p
    ref={ref}
    className={cn("text-sm text-slate-500 dark:text-slate-400", className)}
    {...props}
  />
))
CardDescription.displayName = "CardDescription"

const CardContent = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  // dont allow to words overflow the card
  <div ref={ref} className={cn("p-4 py-2", className)} {...props} />
))
CardContent.displayName = "CardContent"

const CardFooter = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("flex items-end p-4 px-6 pt-0", className)}
    {...props}
  />
))
CardFooter.displayName = "CardFooter"

export { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle }
