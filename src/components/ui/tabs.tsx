import * as TabsPrimitive from "@radix-ui/react-tabs"
import * as React from "react"

import { VariantProps, cva } from "class-variance-authority"
import { cn } from "lib/utils"

const Tabs = TabsPrimitive.Root

const tabsListVariants = cva(
  "inline-flex items-center justify-center px-1 bg-gray-500/10",
  {
    variants: {
      color: {
        primary: "text-primary",
        secondary: "text-secondary",
        error: "text-error",
        success: "text-success",
        warning: "text-warning",
        info: "text-info",
        muted: "bg-gray-500"
      },
      round: {
        none: "rounded-none",
        both: "rounded-full",
        left: "rounded-l-full",
        right: "rounded-r-full",
        t: "rounded-t",
        b: "rounded-b",
        tl: "rounded-tl",
        tr: "rounded-tr",
        bl: "rounded-bl",
        br: "rounded-br"
      },
      radius: {
        full: "rounded-full",
        xs: "rounded-[2px]",
        sm: "rounded-[4px]",
        md: "rounded-[6px]",
        lg: "rounded-[8px]",
        xl: "rounded-[12px]"
      },
      size: {
        xs: "h-4",
        sm: "h-6",
        md: "h-7",
        lg: "px-1.5 h-9",
        xl: "px-2 h-10"
      }
    },
    defaultVariants: {
      size: "md",
      radius: "md",
      round: "both",
      color: "primary"
    }
  }
)

export interface TabsListProps
  extends Omit<
      React.ComponentPropsWithoutRef<typeof TabsPrimitive.List>,
      "color"
    >,
    VariantProps<typeof tabsListVariants> {}

const TabsList = React.forwardRef<
  React.ElementRef<typeof TabsPrimitive.List>,
  TabsListProps
>(({ className, size, round, radius, color, ...props }, ref) => (
  <TabsPrimitive.List
    ref={ref}
    className={cn(
      tabsListVariants({
        color,
        round,
        radius,
        size
      }),
      size !== "xs" && `rounded-${round}-${size}`,
      className
    )}
    {...props}
  />
))
TabsList.displayName = TabsPrimitive.List.displayName

const tabsTriggerVariants = cva(
  "inline-flex items-center bg-transparent hover:text-foreground hover:border-foreground justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow-sm",
  {
    variants: {
      color: {
        primary: "border-primary text-primary",
        secondary: "border-secondary text-secondary",
        error: "border-error text-error",
        success: "border-success text-success",
        warning: "border-warning text-warning",
        info: "border-info text-info",
        muted: "border-gray-500"
      },
      size: {
        xs: "px-2 h-3.5 text-xs",
        sm: "px-3 h-4.5 text-sm",
        md: "px-4 h-5.5 text-base",
        lg: "px-8 h-7.5 text-lg",
        xl: "px-10 h-8.5 text-xl"
      },
      radius: {
        full: "rounded-full",
        xs: "rounded-[2px]",
        sm: "rounded-[4px]",
        md: "rounded-[6px]",
        lg: "rounded-[8px]",
        xl: "rounded-[12px]"
      }
    },
    defaultVariants: {
      color: "muted",
      radius: "sm",
      size: "md"
    }
  }
)

export interface TabsTriggerProps
  extends Omit<
      React.ComponentPropsWithoutRef<typeof TabsPrimitive.Trigger>,
      "color"
    >,
    VariantProps<typeof tabsTriggerVariants> {}

const TabsTrigger = React.forwardRef<
  React.ElementRef<typeof TabsPrimitive.Trigger>,
  TabsTriggerProps
>(({ className, color, radius, size, ...props }, ref) => {
  return (
    <TabsPrimitive.Trigger
      ref={ref}
      className={cn(
        tabsTriggerVariants({
          color,
          radius,
          size
        }),
        className
      )}
      {...props}
    />
  )
})

TabsTrigger.displayName = TabsPrimitive.Trigger.displayName

const TabsContent = React.forwardRef<
  React.ElementRef<typeof TabsPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof TabsPrimitive.Content>
>(({ className, ...props }, ref) => (
  <TabsPrimitive.Content
    ref={ref}
    className={cn(
      "mt-2 ring-offset-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-950 focus-visible:ring-offset-2 dark:ring-offset-slate-950 dark:focus-visible:ring-slate-300",
      className
    )}
    {...props}
  />
))
TabsContent.displayName = TabsPrimitive.Content.displayName

export { Tabs, TabsContent, TabsList, TabsTrigger }
