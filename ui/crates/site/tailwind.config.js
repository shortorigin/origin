// Generated from ui/crates/system_ui/tokens/tokens.toml
const plugin = require("tailwindcss/plugin");

module.exports = {
  content: ["./src/**/*.rs", "./src/**/*.html"],
  theme: {
    extend: {
      colors: {
        semantic: {
          text: {
            primary: "var(--origin-semantic-text-primary)",
            secondary: "var(--origin-semantic-text-secondary)",
            muted: "var(--origin-semantic-text-muted)",
            inverse: "var(--origin-semantic-text-inverse)",
          },
          border: {
            standard: "var(--origin-semantic-border-standard)",
            focus: "var(--origin-semantic-border-focus)",
            selected: "var(--origin-semantic-border-selected)",
          },
          surface: {
            taskbar: "var(--origin-semantic-surface-taskbar-background)",
            window: "var(--origin-semantic-surface-window-background)",
            windowActive: "var(--origin-semantic-surface-window-active-background)",
            menu: "var(--origin-semantic-surface-menu-background)",
            modal: "var(--origin-semantic-surface-modal-background)",
          },
          control: {
            neutral: "var(--origin-semantic-control-neutral-background)",
            accent: "var(--origin-semantic-control-accent-background)",
            danger: "var(--origin-semantic-control-danger-background)",
          },
          state: {
            hover: "var(--origin-semantic-state-hover-surface)",
            active: "var(--origin-semantic-state-active-surface)",
            selected: "var(--origin-semantic-state-selected-surface)",
            focusRing: "var(--origin-semantic-state-focus-ring)",
          },
        },
      },
      spacing: {
        0: "var(--origin-raw-space-0)",
        2: "var(--origin-raw-space-2)",
        4: "var(--origin-raw-space-4)",
        8: "var(--origin-raw-space-8)",
        12: "var(--origin-raw-space-12)",
        16: "var(--origin-raw-space-16)",
        20: "var(--origin-raw-space-20)",
        24: "var(--origin-raw-space-24)",
        28: "var(--origin-raw-space-28)",
        32: "var(--origin-raw-space-32)",
        40: "var(--origin-raw-space-40)",
        48: "var(--origin-raw-space-48)",
      },
      borderRadius: {
        shellSm: "var(--origin-raw-radius-8)",
        shellMd: "var(--origin-raw-radius-12)",
        shellLg: "var(--origin-raw-radius-16)",
        round: "var(--origin-raw-radius-round)",
      },
      boxShadow: {
        embedded: "var(--origin-semantic-layer-embedded-shadow)",
        raised: "var(--origin-semantic-layer-raised-shadow)",
        floating: "var(--origin-semantic-layer-floating-shadow)",
        modal: "var(--origin-semantic-layer-modal-shadow)",
      },
      zIndex: {
        wallpaper: "var(--origin-semantic-layer-wallpaper)",
        desktopBackdrop: "var(--origin-semantic-layer-desktop-backdrop)",
        taskbar: "var(--origin-semantic-layer-taskbar)",
        windows: "var(--origin-semantic-layer-windows)",
        menus: "var(--origin-semantic-layer-menus)",
        modal: "var(--origin-semantic-layer-modal)",
      },
      transitionDuration: {
        fast: "var(--origin-raw-motion-duration-fast)",
        DEFAULT: "var(--origin-raw-motion-duration-standard)",
        slow: "var(--origin-raw-motion-duration-slow)",
      },
      transitionTimingFunction: {
        standard: "var(--origin-raw-motion-easing-standard)",
        emphasized: "var(--origin-raw-motion-easing-emphasized)",
      },
    },
  },
  plugins: [
    plugin(function ({ addUtilities }) {
      addUtilities({
        ".shell-focus-ring": {
          boxShadow: "0 0 0 var(--origin-raw-border-focus-ring-width) var(--origin-semantic-state-focus-ring)",
        },
      });
    }),
  ],
  corePlugins: { preflight: false },
};
