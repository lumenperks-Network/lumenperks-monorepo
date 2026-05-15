import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "LumenPerks",
  description: "Customer loyalty rewards on Stellar",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
