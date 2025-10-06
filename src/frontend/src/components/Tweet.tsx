import * as React from "react";

/**
 * Tweet embed component using Twitter's official widget API
 * @param tweetId - The numeric ID of the tweet to embed
 */
export const Tweet = ({ tweetId }: { tweetId: string }) => {
    const containerRef = React.useRef<HTMLDivElement>(null);
    const [loaded, setLoaded] = React.useState(false);
    const [error, setError] = React.useState(false);

    React.useEffect(() => {
        const container = containerRef.current;
        if (!container) return;

        // Check if Twitter widgets library is available
        const twttr = (window as any).twttr;
        if (!twttr) {
            console.warn("Twitter widgets library not loaded");
            setError(true);
            return;
        }

        // Clear container
        container.innerHTML = "";

        // Detect if CrumbEatr is using a dark theme
        // Dark themes have dark backgrounds, light theme has light background
        const isDarkTheme = window.user?.settings?.theme
            ? [
                  "black",
                  "dark",
                  "calm",
                  "classic",
                  "midnight",
                  "dracula",
                  "abyss",
              ].includes(window.user.settings.theme)
            : true; // Default to dark if no user settings

        // Create tweet embed
        twttr.widgets
            .createTweet(tweetId, container, {
                conversation: "none", // Hide conversation thread
                theme: isDarkTheme ? "dark" : "light", // Match CrumbEatr's theme
            })
            .then((el: any) => {
                if (el) {
                    setLoaded(true);
                    setError(false);
                } else {
                    // Tweet doesn't exist or is deleted
                    setError(true);
                }
            })
            .catch((err: Error) => {
                console.error("Failed to render tweet:", err);
                setError(true);
            });

        // Cleanup function
        return () => {
            if (container) {
                container.innerHTML = "";
            }
        };
    }, [tweetId]); // Only depend on tweetId

    if (error) {
        return (
            <div className="tweet-error">Tweet unavailable (ID: {tweetId})</div>
        );
    }

    return (
        <div
            ref={containerRef}
            className="tweet-embed"
            style={{ minHeight: loaded ? "auto" : "200px" }}
        />
    );
};
