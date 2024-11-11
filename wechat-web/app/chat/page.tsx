'use client'

import { useState, useEffect, useRef } from 'react'
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { Send, Loader2 } from "lucide-react"

type Message = {
  id: number
  text: string
  sender: 'user' | 'bot'
}

export default function Component() {
  const [messages, setMessages] = useState<Message[]>([
    { id: 1, text: "Hello! How can I help you today?", sender: 'bot' },
  ])
  const [inputValue, setInputValue] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const scrollAreaRef = useRef<HTMLDivElement>(null)
  const bottomRef = useRef<HTMLDivElement>(null)
  const [scrollButtomFlag, setScrollButtomFlag] = useState(true)

  useEffect(() => {
    if (scrollButtomFlag) {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' })
        // setScrollTopFlag(false)
    }
  }, [messages])

  const handleSendMessage = () => {
    setScrollButtomFlag(true)
    if (inputValue.trim()) {
      const newMessage: Message = {
        id: Date.now(),
        text: inputValue,
        sender: 'user'
      }
      setMessages(prevMessages => [...prevMessages, newMessage])
      setInputValue('')
      
      // Simulate bot response
      setTimeout(() => {
        const botResponse: Message = {
          id: Date.now() + 1,
          text: "Thanks for your message. I'm a demo bot, so I can't provide a real response.",
          sender: 'bot'
        }
        setMessages(prevMessages => [...prevMessages, botResponse])
      }, 1000)
    }
  }

  const loadMoreMessages = () => {
    setIsLoading(true)
    setScrollButtomFlag(false)
    // Simulate loading more messages
    setTimeout(() => {
      const oldMessages: Message[] = [
        { id: Date.now() - 1000, text: "This is an older message.", sender: 'user' },
        { id: Date.now() - 2000, text: "Here's another old message.", sender: 'bot' },
      ]
      setMessages(prevMessages => [...oldMessages, ...prevMessages])
      setIsLoading(false)
    }, 1000)
  }

  return (
    <div className="flex flex-col h-screen mx-auto">
      <header className="bg-primary text-primary-foreground p-4">
        <h1 className="text-2xl font-bold">Michael</h1>
      </header>
      
      <ScrollArea className="flex-grow p-4" ref={scrollAreaRef}>
        <div className="space-y-4">
          <Button 
            onClick={loadMoreMessages} 
            disabled={isLoading}
            variant="outline"
            className="w-full mb-4"
          >
            {isLoading ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                Loading...
              </>
            ) : (
              'Load More'
            )}
          </Button>
          {messages.map((message) => (
            <div
              key={message.id}
              className={`flex ${message.sender === 'user' ? 'justify-end' : 'justify-start'}`}
            >
              <div className={`flex items-start gap-2.5 ${message.sender === 'user' ? 'flex-row-reverse' : 'flex-row'}`}>
                <Avatar className={message.sender === 'user' ? 'bg-primary' : 'bg-secondary'}>
                  <AvatarFallback>{message.sender === 'user' ? 'U' : 'B'}</AvatarFallback>
                </Avatar>
                <div className={`p-3 rounded-lg ${
                  message.sender === 'user' 
                    ? 'bg-primary text-primary-foreground' 
                    : 'bg-secondary text-secondary-foreground'
                }`}>
                  <p className="text-sm">{message.text}</p>
                </div>
              </div>
            </div>
          ))}
          <div ref={bottomRef} />
        </div>
      </ScrollArea>
      
      <div className="p-4 border-t">
        <form 
          onSubmit={(e) => {
            e.preventDefault()
            handleSendMessage()
          }}
          className="flex gap-2"
        >
          <Input
            type="text"
            placeholder="Type a message..."
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            className="flex-grow"
          />
          <Button type="submit" size="icon">
            <Send className="h-4 w-4" />
            <span className="sr-only">Send message</span>
          </Button>
        </form>
      </div>
    </div>
  )
}