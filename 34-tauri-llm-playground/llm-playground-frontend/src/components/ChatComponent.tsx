
interface ChatMessage {
    role: string;
    content: string;
}

interface ChatResponse {
    message: string;
}

const ChatComponent: React.FC = () => {
    return (
        <div>
            <h1>Chat Component</h1>
        </div>
    );
};


export default ChatComponent;
