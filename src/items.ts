export const Items = [
  "Invalid Item",
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

export function getItem(value: number | null | undefined): string | null {
  if (value === null || value === undefined) {
    return null;
  }
  return Items.at(value) ?? "" + value;
}
