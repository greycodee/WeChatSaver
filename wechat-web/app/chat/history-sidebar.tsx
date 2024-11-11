import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
  } from "@/components/ui/sidebar"
  import { Calendar, Home, Inbox, Search, Settings,Info,MessageSquare } from "lucide-react"
  import { SidebarTrigger } from "@/components/ui/sidebar"

export default function ChatHistorySidebar() {
    return (
    <Sidebar collapsible="none">
      <SidebarContent>
          <SidebarGroup className="px-0">
            <SidebarGroupLabel>Chat History</SidebarGroupLabel>
            <SidebarGroupContent className="px-0">
            <a
                  href="#"
                  className="flex flex-col items-start gap-2 whitespace-nowrap border-b p-4 text-sm leading-tight last:border-b-0 hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
                >
                  <div className="flex w-full items-center gap-2">
                    <span>asd</span>{" "}
                    <span className="ml-auto text-xs">asd</span>
                  </div>
                  <span className="font-medium">asd</span>
                  <span className="line-clamp-2 w-[260px] whitespace-break-spaces text-xs">
                    asd
                  </span>
                </a>
                  
                <a
                  href="#"
                  className="flex flex-col items-start gap-2 whitespace-nowrap border-b p-4 text-sm leading-tight last:border-b-0 hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
                >
                  <div className="flex w-full items-center gap-2">
                    <span>asd</span>{" "}
                    <span className="ml-auto text-xs">asd</span>
                  </div>
                  <span className="font-medium">asd</span>
                  <span className="line-clamp-2 w-[260px] whitespace-break-spaces text-xs">
                    asd
                  </span>
                </a>
                <a
                  href="#"
                  className="flex flex-col items-start gap-2 whitespace-nowrap border-b p-4 text-sm leading-tight last:border-b-0 hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
                >
                  <div className="flex w-full items-center gap-2">
                    <span>asd</span>{" "}
                    <span className="ml-auto text-xs">asd</span>
                  </div>
                  <span className="font-medium">asd</span>
                  <span className="line-clamp-2 w-[260px] whitespace-break-spaces text-xs">
                    asd
                  </span>
                </a>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
    );
}