"use client";

import { Calendar, Home, Inbox, Search, Settings,Info,MessageSquare } from "lucide-react"
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
  import { usePathname, useRouter } from 'next/navigation'
  import React, { useState, useEffect } from 'react';


  const items = [
    {
      title: "Home",
      url: "/",
      icon: Home,
    },
    {
      title: "Chat",
      url: "/chat",
      icon: MessageSquare,
    },
    {
        title: "About",
        url: "/about",
        icon: Info,
      },
  ]
  
  export function AppSidebar() {
    const router = useRouter();
    const pathname = usePathname()
    const [selected, setSelected] = useState<string>(pathname);

    const handleSelect = (path: string) => {
        console.log(path);
        console.log(pathname);
        setSelected(path);
        router.push(path);
    };

    return (
      <Sidebar collapsible="icon">
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel>Application</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {items.map((item) => (
                  <SidebarMenuItem key={item.title}
                    // className={`${selected === item.url ? 'bg-[#f2f2f2] dark:bg-[#1a1a1a] text-primary dark:text-primary' : ''}`}
                  >
                    <SidebarMenuButton 
                    onClick={() => handleSelect(item.url)} 
                    isActive={selected === item.url}
                    
                    asChild>
                      <span>
                        <item.icon />
                        <span>{item.title}</span>
                        </span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
  
    )
  }
  