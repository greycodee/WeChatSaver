import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
} from "@/components/ui/sidebar";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import React from "react";

const ChatHistorySidebar = React.forwardRef<HTMLDivElement, React.ComponentProps<"div">>(
  ({ className, ...props }, ref) => {

    const contacts = [
      { name: "John Doe", message: "Hey, how are you?", time: "2:30 PM" },
      { name: "Jane Smith", message: "Let's catch up later.", time: "1:15 PM" },
      { name: "Alice Johnson", message: "Meeting at 3 PM.", time: "12:45 PM" },
  ];

    return(
      <Sidebar collapsible="none"
          className={className}
          ref={ref}
          {...props}
        >
            <SidebarContent>
                <SidebarGroup className="px-0">
                    <SidebarGroupLabel>Chat History</SidebarGroupLabel>
                    <SidebarGroupContent className="px-0">
                        {contacts.map((contact, index) => (
                            <a
                                key={index}
                                href="#"
                                className="flex items-center gap-4 p-4 mb-4 bg-white rounded-lg shadow-md hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
                            >
                                <Avatar className="w-10 h-10">
                                    <AvatarImage src={`https://i.pravatar.cc/150?img=${index + 1}`} alt={contact.name} />
                                    <AvatarFallback>{contact.name.charAt(0)}</AvatarFallback>
                                </Avatar>
                                <div className="flex flex-col w-full">
                                    <div className="flex justify-between w-full">
                                        <span className="font-medium">{contact.name}</span>
                                        <span className="text-xs text-gray-500">{contact.time}</span>
                                    </div>
                                    <span className="line-clamp-2 text-xs text-gray-600">{contact.message}</span>
                                </div>
                            </a>
                        ))}
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
        </Sidebar>
    )
  }
)
export { ChatHistorySidebar }

// export default function ChatHistorySidebar({ className }) {
    // const contacts = [
    //     { name: "John Doe", message: "Hey, how are you?", time: "2:30 PM" },
    //     { name: "Jane Smith", message: "Let's catch up later.", time: "1:15 PM" },
    //     { name: "Alice Johnson", message: "Meeting at 3 PM.", time: "12:45 PM" },
    // ];

//     return (
        
//     );
// }