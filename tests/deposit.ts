import {
  http,
  defineChain,
  createWalletClient,
  createPublicClient,
  getAddress,
  parseAbi,
  prepareEncodeFunctionData, encodeFunctionData,
  zeroAddress,
  parseUnits,
  parseSignature,
  Address,
  Hex,
  Hash,
  getContract,
  toHex,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";

const relayerEndpoint = "http://localhost:3000";
const tangle = defineChain({
  id: 3287,
  name: "Tangle Testnet",
  nativeCurrency: {
    decimals: 18,
    name: "Tangle Test Network Token",
    symbol: "TTNT",
  },
  rpcUrls: {
    default: {
      http: ["http://localhost:9944"],
      webSocket: ["ws://localhost:9944"],
    },
    remote: {
      http: ["https://testnet-rpc.tangle.tools"],
      webSocket: ["wss://testnet-rpc.tangle.tools"],
    },
  },
  blockExplorers: {
    default: { name: "Explorer", url: "http://localhost:3000" },
    remote: { name: "Explorer", url: "https://testnet-explorer.tangle.tools" },
  },
  contracts: {
    permit: {
      address: getAddress('0x0000000000000000000000000000000000000805'),
      blockCreated: 0,
    },
    multiAssetDelegation: {
      address: getAddress('0x0000000000000000000000000000000000000822'),
      blockCreated: 0,
    },
  }
});

const BOB = privateKeyToAccount(
  "0x79c3b7fc0b7697b9414cb87adcb37317d1cab32818ae18c0e97ad76395d1fdcf",
);

const walletClient = createWalletClient({
  account: BOB,
  chain: tangle,
  transport: http(),
});

const mad = parseAbi([
  'function deposit(uint256 assetId, address tokenAddress, uint256 amount, uint8 lockMultiplier) external',
]);
const callPermitAbi = parseAbi([
  'function dispatch(address from, address to, uint256 value, bytes memory data, uint64 gaslimit, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external returns (bytes memory output)',
  'function nonces(address owner) external view returns (uint256)',
]);

const callPermit = getContract({
  address: tangle.contracts.permit.address,
  abi: callPermitAbi,
  client: walletClient,
})

const domain = {
  name: 'Call Permit Precompile',
  version: '1',
  chainId: BigInt(tangle.id),
  verifyingContract: tangle.contracts.permit.address,
} as const

// The named list of all type definitions
const types = {
  EIP712Domain: [
    {
      name: 'name',
      type: 'string',
    },
    {
      name: 'version',
      type: 'string',
    },
    {
      name: 'chainId',
      type: 'uint256',
    },
    {
      name: 'verifyingContract',
      type: 'address',
    },
  ],
  CallPermit: [
    {
      name: 'from',
      type: 'address',
    },
    {
      name: 'to',
      type: 'address',
    },
    {
      name: 'value',
      type: 'uint256',
    },
    {
      name: 'data',
      type: 'bytes',
    },
    {
      name: 'gaslimit',
      type: 'uint64',
    },
    {
      name: 'nonce',
      type: 'uint256',
    },
    {
      name: 'deadline',
      type: 'uint256',
    },
  ],
} as const

const deposit = prepareEncodeFunctionData({
  abi: mad,
  functionName: 'deposit',
})

const FROM = BOB;
const TO = tangle.contracts.multiAssetDelegation.address;
const VALUE = 0n;
const ASSET_ID = 0n; // TNT
const AMOUNT = parseUnits('10', tangle.nativeCurrency.decimals)
const LOCK_MULTIPLIER = 0;
const GAS_LIMIT = 600000n;

const nonce = await callPermit.read.nonces([FROM.address]);
const now = (Date.now() / 1000) | 0; // Unix timestamp in seconds
const deadline = BigInt(now) + 60n * 60n; // 1 hour
const data =
  encodeFunctionData({
    ...deposit,
    args: [ASSET_ID, zeroAddress, AMOUNT, LOCK_MULTIPLIER],
  });


const signature = await walletClient.signTypedData({
  account: FROM,
  domain,
  types,
  primaryType: 'CallPermit',
  message: {
    from: FROM.address,
    to: TO,
    value: VALUE,
    gaslimit: GAS_LIMIT,
    nonce,
    deadline,
    data,
  },
})

const parsedSignature = parseSignature(signature);

type RelayTransactionRequest = {
  from: Address;
  to: Address;
  value: Hex; // Bigint as hex string
  data: Hex;
  gaslimit: number;
  deadline: Hex; // Bigint as hex string
  v: number;
  r: Hex;
  s: Hex;
};

type RelayTransactionResponseSuccess = {
  status: 'success';
  txHash: Hash;
  simulatedOutcome: Hex;
};

type RelayTransactionResponseFailure = {
  status: 'failure';
  error: string;
  details: string | undefined;
};

type RelayTransactionResponse = RelayTransactionResponseSuccess | RelayTransactionResponseFailure;

// Prepare the Request to the relayer.

const request: RelayTransactionRequest = {
  from: FROM.address,
  to: TO,
  value: toHex(VALUE),
  data,
  gaslimit: Number(GAS_LIMIT),
  deadline: toHex(deadline),
  v: Number(parsedSignature.v!),
  r: parsedSignature.r,
  s: parsedSignature.s,
};

// Send the request to the relayer.

const reqBody = JSON.stringify(request, null, 2);
console.log(`Request Body: ${reqBody}`);
const response = await fetch(`${relayerEndpoint}/api/v1/relay`, {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: reqBody,
});

const relayResponse: RelayTransactionResponse = await response.json();
console.log(`Response: ${JSON.stringify(relayResponse, null, 2)}`);
// Handle the response from the relayer.

if (relayResponse.status === 'success') {
  console.log(`Transaction Hash: ${relayResponse.txHash}`);
  console.log(`Simulated Outcome: ${relayResponse.simulatedOutcome}`);
} else if (relayResponse.status === 'failure') {
  console.error(`Error: ${relayResponse.error}`);
  console.error(`Details: ${relayResponse.details}`);
}
