export const Items = [
  "🐈",
  "🐈‍⬛",
  "😺",
  "😹",
  "😻",
  "🙀",
  "🐅",
  "🐯",
  "🦁",
  "🌰",
  "🦊",
  "🐚",
  "🪶",
  "🦴",
  "🎈",
  "🎃",
  "🎁",
  "🧶",
  "👑",
  "💎",
  "💍",
  "🏅",
  "🏆",
  "🗝️",
  "🗡️",
  "🍔",
  "🥠",
  "🥢",
  "🥒",
  "🌟",
];

export function getItemNonZeroU8(
  value: number | null | undefined
): string | null {
  if (value === null || value === undefined || value === 0 || value < 0) {
    return null;
  }
  if (value > Items.length) {
    return "" + value;
  }
  return Items[value - 1];
}
