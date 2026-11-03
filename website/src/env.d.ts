/// <reference types="vite/client" />

declare module 'virtual:cc-docs-manifest' {
  const manifest: { key: string; title: string }[]
  export default manifest
}

declare module 'virtual:cc-docs-search' {
  export const corpusUrl: string
}
