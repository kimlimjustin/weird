<script lang="ts">
	import { env } from '$env/dynamic/public';
	import Avatar from '$lib/components/avatar/view.svelte';
	import type { PageData } from './$types';
	const { data }: { data: PageData } = $props();
	const profile = data.profile;
</script>

<svelte:head>
	<title>
		{!('error' in profile) ? profile.display_name || profile.username : 'Profile'} | {env.PUBLIC_INSTANCE_NAME}
	</title>
</svelte:head>

{#if !('error' in profile)}
	<main class="flex flex-col items-center">
		<div class="card mt-12 flex w-[600px] max-w-[90%] flex-col gap-4 p-8 text-xl">
			<div class="flex items-center gap-4">
				<Avatar username={profile.username} />
				<h1 class="my-3 text-4xl">{profile.display_name || profile.username}</h1>
			</div>

			<hr class="mb-4" />

			<div class="flex flex-col gap-4">
				{#if profile.bio}
					<div>
						<pre
							class="mt-2 text-wrap rounded-lg bg-surface-300 p-3 font-sans text-base dark:bg-surface-900">{profile.bio}</pre>
					</div>
				{/if}
				{#if profile.links}
					{#each profile.links as link}
						<a class="variant-ghost btn" href={link.url}>
							{link.label || link.url}
						</a>
					{/each}
				{/if}
				{#if profile.tags && profile.tags.length > 0}
					<div class="flex items-center gap-2">
						<strong>Tags: </strong>
						<span class="flex flex-wrap gap-2 text-base">
							{#each profile.tags as tag}
								<a
									class="text-surface-900-50-token btn rounded-md bg-surface-200 p-1 hover:bg-surface-400 dark:bg-surface-900 dark:text-surface-100 dark:hover:bg-surface-700"
									href={`/members?q=${tag}`}
								>
									{tag}
								</a>
							{/each}
						</span>
					</div>
				{/if}
				<!-- {#if profile.mastodon_profile}
					<a
						class="variant-ghost btn"
						href={`/u/${profile.username}/mastodon`}
					>
						View Mastodon Profile
					</a>
				{/if} -->
				{#if data.pages.length > 0}
					<h3 class="mt-4 text-center text-2xl font-bold">Pages</h3>
					{#each data.pages as page}
						<a class="variant-ghost btn font-mono" href={`/a/${profile.username}/${page}`}>
							{page}
						</a>
					{/each}
				{/if}
			</div>
		</div>
	</main>
{:else}
	<main class="flex flex-col items-center">
		<aside class="alert variant-ghost-error mt-8 w-80">
			<div class="alert-message">
				<p>Error loading user: {profile.error}</p>
			</div>
		</aside>
	</main>
{/if}
