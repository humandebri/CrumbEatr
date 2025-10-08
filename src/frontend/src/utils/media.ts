/**
 * URL detection utilities for media embeds
 */

/**
 * Regex to match Twitter/X status URLs
 * Matches both twitter.com and x.com domains
 * Captures tweet ID in group 3
 */
export const twitterLinkRegex = (): RegExp =>
    /https?:\/\/(twitter|x)\.com\/[^/]+\/status(es)?\/(\d+)[?^ ]*/i;

/**
 * Extract tweet ID from a Twitter/X URL
 * @param url - The URL to parse
 * @returns The tweet ID if found, null otherwise
 */
export const extractTweetId = (url: string): string | null => {
    const match = url.match(twitterLinkRegex());
    return match ? match[3] : null;
};
