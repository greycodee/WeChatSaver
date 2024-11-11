import ChatHistorySidebar from "./history-sidebar";


export default function ChatLayout({children}: Readonly<{children: React.ReactNode;}>) {
  return (
    <div className="flex h-screen">
      <ChatHistorySidebar />
      <main className="w-full h-full p-0 min-w-96">
        {children}
      </main>
    </div>
  );

}