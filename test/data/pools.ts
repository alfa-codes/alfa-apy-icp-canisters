// Token constants
export const TOKENS = {
  ICP: "ryjl3-tyaaa-aaaaa-aaaba-cai",
  CKUSDT: "cngnf-vqaaa-aaaar-qag4q-cai",
  CKBTC: "mxzaz-hqaaa-aaaar-qaada-cai",
  PANDA: "druyg-tyaaa-aaaaq-aactq-cai",
  NFIDW: "mih44-vaaaa-aaaaq-aaekq-cai",
  ICS: "ca6gz-lqaaa-aaaaq-aacwa-cai",
  CKETH: "ss2fx-dyaaa-aaaar-qacoq-cai",
  GLDT: "6c7su-kiaaa-aaaar-qaira-cai",
  CKLINK: "g4tto-rqaaa-aaaar-qageq-cai"
} as const;

export const PROVIDERS = {
  KongSwap: "KongSwap",
  ICPSwap: "ICPSwap"
} as const;

// Pool data
export const POOLS = [
  // // Strategy 3: ICP-ckUSDT Dynamic Strategy
  // {
  //   id: `${PROVIDERS.KongSwap}_${TOKENS.ICP}_${TOKENS.CKUSDT}`, // KongSwap_ryjl3-tyaaa-aaaaa-aaaba-cai_cngnf-vqaaa-aaaar-qag4q-cai
  //   provider: PROVIDERS.KongSwap,
  //   token0: TOKENS.ICP,
  //   token1: TOKENS.CKUSDT
  // },
  // {
  //   id: `${PROVIDERS.ICPSwap}_${TOKENS.CKUSDT}_${TOKENS.ICP}`, // ICPSwap_cngnf-vqaaa-aaaar-qag4q-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
  //   provider: PROVIDERS.ICPSwap,
  //   token0: TOKENS.CKUSDT,
  //   token1: TOKENS.ICP
  // },

  // // Strategy 4: Panda-ICP Balanced Strategy
  // {
  //   id: `${PROVIDERS.KongSwap}_${TOKENS.PANDA}_${TOKENS.ICP}`, // KongSwap_druyg-tyaaa-aaaaq-aactq-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
  //   provider: PROVIDERS.KongSwap,
  //   token0: TOKENS.PANDA,
  //   token1: TOKENS.ICP
  // },
  // {
  //   id: `${PROVIDERS.ICPSwap}_${TOKENS.PANDA}_${TOKENS.ICP}`, // ICPSwap_druyg-tyaaa-aaaaq-aactq-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
  //   provider: PROVIDERS.ICPSwap,
  //   token0: TOKENS.PANDA,
  //   token1: TOKENS.ICP
  // },

  // // Strategy 5: ICS-ICP Balanced Strategy
  // {
  //   id: `${PROVIDERS.KongSwap}_${TOKENS.ICS}_${TOKENS.ICP}`, // KongSwap_ca6gz-lqaaa-aaaaq-aacwa-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
  //   provider: PROVIDERS.KongSwap,
  //   token0: TOKENS.ICS,
  //   token1: TOKENS.ICP
  // },
  // {
  //   id: `${PROVIDERS.ICPSwap}_${TOKENS.ICS}_${TOKENS.ICP}`, // ICPSwap_ca6gz-lqaaa-aaaaq-aacwa-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
  //   provider: PROVIDERS.ICPSwap,
  //   token0: TOKENS.ICS,
  //   token1: TOKENS.ICP
  // },

  // // Strategy 6: ckBTC-ckUSDT Balanced Strategy
  // {
  //   id: `${PROVIDERS.KongSwap}_${TOKENS.CKBTC}_${TOKENS.CKUSDT}`, // KongSwap_mxzaz-hqaaa-aaaar-qaada-cai_cngnf-vqaaa-aaaar-qag4q-cai
  //   provider: PROVIDERS.KongSwap,
  //   token0: TOKENS.CKBTC,
  //   token1: TOKENS.CKUSDT
  // },
  // {
  //   id: `${PROVIDERS.ICPSwap}_${TOKENS.CKUSDT}_${TOKENS.CKBTC}`, // ICPSwap_cngnf-vqaaa-aaaar-qag4q-cai_mxzaz-hqaaa-aaaar-qaada_cai
  //   provider: PROVIDERS.ICPSwap,
  //   token0: TOKENS.CKUSDT,
  //   token1: TOKENS.CKBTC
  // },

  // Strategy 7: ICP-ckETH Dynamic Strategy
  {
    id: `${PROVIDERS.KongSwap}_${TOKENS.CKETH}_${TOKENS.ICP}`, // KongSwap_ss2fx-dyaaa-aaaar-qacoq-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
    provider: PROVIDERS.KongSwap,
    token0: TOKENS.CKETH,
    token1: TOKENS.ICP
  },
  {
    id: `${PROVIDERS.ICPSwap}_${TOKENS.ICP}_${TOKENS.CKETH}`, // ICPSwap_ryjl3-tyaaa-aaaaa-aaaba-cai_ss2fx-dyaaa-aaaar-qacoq-cai
    provider: PROVIDERS.ICPSwap,
    token0: TOKENS.ICP,
    token1: TOKENS.CKETH
  },

  // // Strategy 8: ckBTC-ICP Dynamic Strategy
  // {
  //   id: `${PROVIDERS.KongSwap}_${TOKENS.CKBTC}_${TOKENS.ICP}`, // KongSwap_mxzaz-hqaaa-aaaar-qaada-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
  //   provider: PROVIDERS.KongSwap,
  //   token0: TOKENS.CKBTC,
  //   token1: TOKENS.ICP
  // },
  // {
  //   id: `${PROVIDERS.ICPSwap}_${TOKENS.CKBTC}_${TOKENS.ICP}`, // ICPSwap_mxzaz-hqaaa-aaaar-qaada-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
  //   provider: PROVIDERS.ICPSwap,
  //   token0: TOKENS.CKBTC,
  //   token1: TOKENS.ICP
  // },

  // Strategy 9: GLDT-ckUSDT Balanced Strategy
  {
    id: `${PROVIDERS.KongSwap}_${TOKENS.GLDT}_${TOKENS.CKUSDT}`, // KongSwap_6c7su-kiaaa-aaaar-qaira-cai_cngnf-vqaaa-aaaar-qag4q-cai
    provider: PROVIDERS.KongSwap,
    token0: TOKENS.GLDT,
    token1: TOKENS.CKUSDT
  },
  // {
  //   id: `${PROVIDERS.ICPSwap}_${TOKENS.GLDT}_${TOKENS.CKUSDT}`, // ICPSwap_6c7su-kiaaa-aaaar-qaira-cai_cngnf-vqaaa-aaaar-qag4q-cai
  //   provider: PROVIDERS.ICPSwap,
  //   token0: TOKENS.GLDT,
  //   token1: TOKENS.CKUSDT
  // },

  // // Strategy 10: CKLINK-ICP Balanced Strategy
  // {
  //   id: `${PROVIDERS.KongSwap}_${TOKENS.CKLINK}_${TOKENS.ICP}`, // KongSwap_g4tto-rqaaa-aaaar-qageq-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
  //   provider: PROVIDERS.KongSwap,
  //   token0: TOKENS.CKLINK,
  //   token1: TOKENS.ICP
  // },
  // {
  //   id: `${PROVIDERS.ICPSwap}_${TOKENS.CKLINK}_${TOKENS.ICP}`, // ICPSwap_g4tto-rqaaa-aaaar-qageq-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
  //   provider: PROVIDERS.ICPSwap,
  //   token0: TOKENS.CKLINK,
  //   token1: TOKENS.ICP
  // },


// No success yet
  {
    id: `${PROVIDERS.KongSwap}_${TOKENS.GLDT}_${TOKENS.ICP}`, // KongSwap_6c7su-kiaaa-aaaar-qaira-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
    provider: PROVIDERS.KongSwap,
    token0: TOKENS.GLDT,
    token1: TOKENS.ICP
  },
  {
    id: `${PROVIDERS.ICPSwap}_${TOKENS.GLDT}_${TOKENS.ICP}`, // ICPSwap_6c7su-kiaaa-aaaar-qaira-cai_ryjl3-tyaaa-aaaaa-aaaba-cai
    provider: PROVIDERS.ICPSwap,
    token0: TOKENS.GLDT,
    token1: TOKENS.ICP
  },




] as const;

// Types for better TypeScript support
export type TokenName = keyof typeof TOKENS;
export type Pool = typeof POOLS[number];
export type Provider = typeof PROVIDERS[keyof typeof PROVIDERS];
