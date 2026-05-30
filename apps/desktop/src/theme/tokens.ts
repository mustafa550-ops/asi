export const tokens = {
  colors: {
    bg: { primary: "#0d1117", secondary: "#161b22", tertiary: "#21262d" },
    border: { default: "#30363d", muted: "#21262d", accent: "#58a6ff" },
    text: { primary: "#c9d1d9", secondary: "#8b949e", accent: "#58a6ff" },
    status: {
      success: "#3fb950",
      warning: "#d29922",
      error: "#f85149",
      info: "#58a6ff",
    },
    btn: { primary: "#1f6feb", success: "#238636", danger: "#da3633" },
  },
  spacing: (n: number) => `${n * 4}px`,
  radius: { sm: "4px", md: "6px", lg: "8px", xl: "12px" },
  shadow: {
    card: "0 1px 3px rgba(0,0,0,0.3)",
    modal: "0 8px 32px rgba(0,0,0,0.5)",
  },
  fontSize: {
    xs: "0.7rem",
    sm: "0.8rem",
    md: "0.9rem",
    lg: "1.1rem",
    xl: "1.4rem",
  },
  transition: { fast: "0.15s ease", normal: "0.25s ease" },
} as const;

export type Theme = typeof tokens;
