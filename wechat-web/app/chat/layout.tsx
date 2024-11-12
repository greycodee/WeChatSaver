import {ChatHistorySidebar} from "./history-sidebar";


export default function ChatLayout({children}: Readonly<{children: React.ReactNode;}>) {
  return (
    <div className="flex h-screen">
      <ChatHistorySidebar  
      className="w-80 p-2 min-w-80" 
      />
      
      <main className="w-full h-full p-0 min-w-96">
        
        {children}
      </main>
    </div>
  );

}