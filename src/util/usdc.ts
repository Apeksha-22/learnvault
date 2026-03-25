/**
 * Utility functions for USDC token operations on Stellar
 */

import { Contract, rpc } from "@stellar/stellar-sdk"

/**
 * Get the USDC contract ID from environment variables
 */
export function getUSDCContractId(): string {
	const contractId = import.meta.env.PUBLIC_USDC_CONTRACT_ID

	if (!contractId) {
		throw new Error(
			"USDC contract ID not configured. Please set PUBLIC_USDC_CONTRACT_ID in your .env file.",
		)
	}

	return contractId
}

/**
 * Mint test USDC tokens to a specified address
 * (Currently uses CLI fallback until contract client is ready)
 */
export async function mintTestUSDC(
	recipientAddress: string,
	amount: number = 1000,
): Promise<void> {
	try {
		const contractId = getUSDCContractId()

		// Convert to stroops safely (7 decimals)
		const amountStroops = BigInt(Math.round(amount * 1e7))

		const rpcUrl =
			import.meta.env.PUBLIC_STELLAR_RPC_URL || "http://localhost:8000/rpc"

		// Setup RPC + Contract (kept for future use)
		const server = new rpc.Server(rpcUrl)
		const contract = new Contract(contractId)

		// 🔴 Placeholder until contract client + signing is implemented
		throw new Error(
			`Minting via UI not implemented yet.\n\n` +
				`Use CLI instead:\n` +
				`./scripts/mint-test-usdc.sh ${recipientAddress} ${amount}\n`,
		)

		// ✅ FUTURE IMPLEMENTATION (example)
		// const tx = await buildTransaction(...)
		// const simulated = await server.simulateTransaction(tx)
		// const signed = signTransaction(tx)
		// await server.sendTransaction(signed)
	} catch (error) {
		if (error instanceof Error) {
			throw error
		}
		throw new Error("Failed to mint test USDC")
	}
}

/**
 * Get USDC balance for an address
 */
export async function getUSDCBalance(address: string): Promise<number> {
	try {
		const contractId = getUSDCContractId()

		const rpcUrl =
			import.meta.env.PUBLIC_STELLAR_RPC_URL || "http://localhost:8000/rpc"

		const server = new rpc.Server(rpcUrl)
		const contract = new Contract(contractId)

		// 🔴 Placeholder until contract client is ready
		throw new Error(
			"Balance checking not implemented yet. Requires contract client integration.",
		)

		// ✅ FUTURE IMPLEMENTATION
		// const result = await contract.call("balance", { id: address })
		// return Number(result) / 1e7
	} catch (error) {
		if (error instanceof Error) {
			throw error
		}
		throw new Error("Failed to get USDC balance")
	}
}
