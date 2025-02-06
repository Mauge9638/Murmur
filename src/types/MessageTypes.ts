export enum MessageSender {
  User = "User",
  Agent = "Agent",
}

/* export type MessageType = {
  agent: MessageSender;
  message: string;
  timestampInMs: number;
}; */

export type Message = AgentMessage | UserMessage;

export type AgentMessage = {
  agent: MessageSender.Agent;
  message: string;
  timestampInMs: number;
};

export type UserMessage = {
  agent: MessageSender.User;
  message: string;
  timestampInMs: number;
};
