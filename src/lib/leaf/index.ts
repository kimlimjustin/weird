import { env } from '$env/dynamic/private';
import '$lib/websocket-polyfill';
import {
	RpcClient,
	type Digest,
	base32Decode,
	base32Encode,
	type PathSegment,
	type ExactLink
} from 'leaf-proto';

/** The Leaf RPC client used to connect to our backend data server. */
export const leafClient = new RpcClient(env.BACKEND_URL, env.BACKEND_SECRET);

/** The communal namespace that all Weird instances live in. */
export const WEIRD_NAMESPACE_SECRET: Digest = new Uint8Array([
	171, 39, 207, 135, 254, 196, 186, 126, 16, 39, 48, 101, 89, 153, 23, 201, 145, 112, 90, 137, 31,
	215, 183, 175, 201, 167, 198, 156, 23, 195, 217, 7
]);

export const WEIRD_NAMESPACE: Digest =
	await leafClient.import_namespace_secret(WEIRD_NAMESPACE_SECRET);

let secret: string | undefined = env.INSTANCE_SUBSPACE_SECRET;

if (!secret) {
	const subspace = await leafClient.create_subspace();
	const key = await leafClient.get_subspace_secret(subspace);
	if (!key) {
		throw 'Error: subspace just created does not exist';
	}
	secret = base32Encode(key);
	console.warn(
		'Warning: INSTANCE_SUBSPACE_SECRET environment variable \
not set, generating a new secret. Set set the environment variable to \
this secret when running the server next, to keep using the same data:',
		secret
	);
}

export let INSTANCE_SUBSPACE: Digest = await leafClient.import_subspace_secret(
	base32Decode(secret)
);

/** Create an ExactLink from a path in this Weird instance. */
export function instance_link(path: PathSegment[]): ExactLink {
	return {
		namespace: WEIRD_NAMESPACE,
		subspace: INSTANCE_SUBSPACE,
		path
	};
}

console.log(`Leaf client initialized:
    Weird Namespace ID        : ${base32Encode(WEIRD_NAMESPACE)}
    Weird Instance Subspace ID: ${base32Encode(INSTANCE_SUBSPACE)}`);