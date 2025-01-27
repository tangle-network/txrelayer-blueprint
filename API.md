## Transaction Relayer API

**API Version: `v1`**

**Deployed at**:

- By Tangle Team:
  - **Testnet**: `https://testnet-txrelayer.tangle.tools`
  - **Mainnet**: TBD

#### Relay Transaction Endpoint

`POST /api/v1/relay`

Relays a transaction with a valid EIP-712 signature through the Call Permit precompile.

##### Request Body

```typescript
{
  from: Address;      // The address initiating the transaction
  to: Address;        // The target contract address
  value: Hex;         // Amount of native tokens to send (in hex)
  data: Hex;          // Encoded function call data
  gaslimit: number;   // Maximum gas allowed for the transaction
  deadline: Hex;      // Timestamp when the signature expires (in hex)
  v: number;          // Recovery ID from the signature
  r: Hex;            // R component of the signature
  s: Hex;            // S component of the signature
}
```

##### Response

The API returns a JSON object with one of the following structures:

**Success Response:**
```typescript
{
  status: 'success';
  txHash: Hash;            // The transaction hash
  simulatedOutcome: Hex;   // The simulated transaction outcome
}
```

**Failure Response:**
```typescript
{
  status: 'failure';
  error: string;           // Error message
  details?: string;        // Additional error details if available
}
```

##### Example Usage

```typescript
const request = {
  from: "0x...",    // Sender's address
  to: "0x...",      // Target contract address
  value: "0x0",     // Amount in hex
  data: "0x...",    // Encoded function data
  gaslimit: 600000, // Gas limit
  deadline: "0x...", // Deadline timestamp in hex
  v: 27,            // Signature v value
  r: "0x...",       // Signature r value
  s: "0x..."        // Signature s value
};

const response = await fetch("http://localhost:3000/api/v1/relay", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify(request),
});

const result = await response.json();
```

##### Notes

- The API expects EIP-712 signed transaction data for the Call Permit precompile.
- The deadline parameter should be a future timestamp to ensure the transaction doesn't expire.
- Gas limit should be set appropriately for the transaction being executed.
- All hex values should be prefixed with "0x".
