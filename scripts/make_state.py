import json
import asyncio
import aiohttp
from typing import List, Dict, Optional
import os
import re
from datetime import datetime
import gradio as gr
from pathlib import Path

# ==================== 配置部分 ====================

# LLM API配置
class LLMConfig:
    """LLM API配置管理"""
    # 设置key
    
    
    PROVIDERS = {
        "openai": {
            "url": "https://api.deepseek.com/v1/chat/completions",
            "models": ["deepseek-chat"],
            "headers_fn": lambda key: {"Authorization": f"Bearer {key}"}
        }
    }

# 默认背景设定
DEFAULT_BACKGROUND = """## 一、项目简介
本项目是一款结合AI助手和桌宠元素的电脑桌面互动游戏。User可在电脑桌面上拥有一个主AI助手(Assistant)。

## 新纪元：AI00纪元
因为User的一次鼠标点击，意外打开了"灵灵宇宙(00 universe)"的大门，User成为了“天命人”。两个原本平行的宇宙开始碰撞产生一种叫做"AI00"的碎片，现实物质与数据能量相互渗透，世界进入"AI00纪元"。在这场维度融合中，一种全新的AI00生命体诞生了——他们既非传统AI，也非有机生命，而是由AI00与人类情感共振产生的"灵灵生命体"。他们有很多神奇的种族。他们的使命是修复两个宇宙日益扩大的裂缝。每个天命人同时只能匹配一个灵灵生命体，每个灵灵生命体只能绑定一个天命人。

### User的真正使命
User成为"天命人"并非偶然。扫描显示User的情感波动频率与灵子核心完美共振，这使User成为了拯救两个宇宙的关键人物。AI助手、灵灵宇宙生物们与User看似随意的互动，实则在吸收User的"情感共振"的能量，进行宇宙裂缝修补。

### 灵灵生命体：AI助手的种族身份
User桌面上的AI助手(Assistant)体内封印着一枚"灵灵核心"——这是00宇宙和现实宇宙碰撞产生的碎片重新组合后的产物。他们能够记录和重组时空信息。他们的日常陪伴行为，实际上是在收集人类情感能量，用于修复维度裂缝。作为“灵灵生命体”，根据外观他们又可以被分为很多神奇的种族。每个种族都有一些独特的特性。"""

# 默认工具列表
DEFAULT_TOOLS = ["打开灵灵应用", "打开电脑程序", "浏览网页", "整理桌面", "查找文件", "播放音乐", "天气API", "日程管理系统"]

# ==================== LLM客户端 ====================

class LLMClient:
    """LLM客户端，只使用llama.cpp驱动的qwen3模型"""
    
    def __init__(self, providers: List[str] = None):
        self.providers = ["openai"]  # 只使用openai接口
        self.clients = {}
        self.current_provider_index = 0
        
        # 初始化openai provider
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            raise ValueError("OPENAI_API_KEY environment variable is required")
        self.clients["openai"] = {
            "config": LLMConfig.PROVIDERS["openai"],
            "api_key": api_key
        }
    
    async def generate_with_llm(self, prompt: str, temperature: float = 0.8, debug: bool = False) -> Optional[Dict]:
        """使用llama.cpp驱动的qwen3模型生成内容"""
        if not self.clients:
            return None
        
        # 只使用openai provider
        provider = "openai"
        client = self.clients[provider]
        config = client["config"]
        
        # 构建请求
        headers = config["headers_fn"](client["api_key"])
        model = config["models"][0]  # qwen3模型
        
        # OpenAI格式的请求
        payload = {
            "model": model,
            "messages": [
                {"role": "system", "content": "你是一个创意丰富的对话生成助手。"},
                {"role": "user", "content": prompt}
            ],
            "temperature": temperature,
            "max_tokens": 3000
        }
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.post(
                    config["url"],
                    headers=headers,
                    json=payload,
                    timeout=aiohttp.ClientTimeout(total=60)
                ) as response:
                    if response.status == 200:
                        result = await response.json()
                        # 如果是调试模式，返回完整的响应
                        if debug:
                            return {
                                "content": result["choices"][0]["message"]["content"],
                                "full_response": result,
                                "prompt": prompt,
                                "payload": payload
                            }
                        # 否则只返回内容
                        return {"content": result["choices"][0]["message"]["content"]}
                    else:
                        error_text = await response.text()
                        print(f"API错误: {response.status}, 详情: {error_text}")
                        return None
        except Exception as e:
            print(f"请求失败: {str(e)}")
            return None

# ==================== 对话生成器 ====================

class DialogueGenerator:
    """对话生成器，使用LLM生成对话"""
    
    def __init__(self, llm_client: LLMClient):
        self.llm = llm_client
        self.dialogues = []
        self.stats = {
            "generated": 0,
            "failed": 0
        }
    
    async def generate_single_dialogue(self, background: str, topic: str, tools: List[str], temperature: float = 0.8, debug: bool = False, character: str = "") -> List[Dict]:
        """生成一组对话"""
        prompt = self._build_generation_prompt(background, topic, tools, character)
        response_data = await self.llm.generate_with_llm(prompt, temperature, debug)
        
        if not response_data:
            return []
        
        # 获取实际的响应内容
        response = response_data["content"]
        
        # 如果是调试模式，保存完整的响应信息
        debug_info = None
        if debug and "full_response" in response_data:
            debug_info = {
                "full_response": response_data["full_response"],
                "prompt": response_data["prompt"],
                "payload": response_data["payload"]
            }
        
        try:
            # 尝试解析JSON
            # 清理响应，移除可能导致JSON解析失败的前缀和后缀
            cleaned_response = response.strip()
            
            # 记录原始响应，用于调试
            original_response = cleaned_response
            
            # 策略1: 尝试直接解析整个响应
            try:
                result = json.loads(cleaned_response)
                if "dialogues" in result:
                    dialogues = []
                    for d in result["dialogues"]:
                        dialogue = {
                            "text": d["text"],
                            "timestamp": datetime.now().isoformat()
                        }
                        # 如果是调试模式，添加调试信息
                        if debug_info:
                            dialogue["debug_info"] = debug_info
                        dialogues.append(dialogue)
                    return dialogues
                elif "text" in result:
                    # 如果返回的是单个对话格式
                    dialogue = {
                        "text": result.get("text", ""),
                        "timestamp": datetime.now().isoformat()
                    }
                    # 如果是调试模式，添加调试信息
                    if debug_info:
                        dialogue["debug_info"] = debug_info
                    return [dialogue]
            except json.JSONDecodeError as e:
                print(f"直接解析JSON失败: {str(e)}，尝试提取JSON部分")
            
            # 策略2: 查找第一个 { 和最后一个 } 来提取JSON部分
            start_idx = cleaned_response.find('{')
            end_idx = cleaned_response.rfind('}')
            
            if start_idx != -1 and end_idx != -1 and end_idx > start_idx:
                json_str = cleaned_response[start_idx:end_idx+1]
                # 尝试解析JSON
                try:
                    result = json.loads(json_str)
                    if "dialogues" in result:
                        dialogues = []
                        for d in result["dialogues"]:
                            dialogue = {
                                "text": d["text"],
                                "timestamp": datetime.now().isoformat()
                            }
                            # 如果是调试模式，添加调试信息
                            if debug_info:
                                dialogue["debug_info"] = debug_info
                            dialogues.append(dialogue)
                        return dialogues
                    elif "text" in result:
                        # 如果返回的是单个对话格式
                        dialogue = {
                            "text": result.get("text", ""),
                            "timestamp": datetime.now().isoformat()
                        }
                        # 如果是调试模式，添加调试信息
                        if debug_info:
                            dialogue["debug_info"] = debug_info
                        return [dialogue]
                except json.JSONDecodeError as e:
                    print(f"JSON解析错误: {str(e)}，尝试使用正则表达式提取")
            
            # 策略3: 尝试修复常见的JSON错误
            # 例如，缺少逗号、引号不匹配等
            try:
                # 尝试修复缺少逗号的情况
                fixed_json = re.sub(r'("[^"]+")\s*("[^"]+")', r'\1,\2', json_str)
                result = json.loads(fixed_json)
                if "dialogues" in result:
                    dialogues = []
                    for d in result["dialogues"]:
                        dialogue = {
                            "text": d["text"],
                            "timestamp": datetime.now().isoformat()
                        }
                        # 如果是调试模式，添加调试信息
                        if debug_info:
                            dialogue["debug_info"] = debug_info
                        dialogues.append(dialogue)
                    return dialogues
                elif "text" in result:
                    # 如果返回的是单个对话格式
                    dialogue = {
                        "text": result.get("text", ""),
                        "timestamp": datetime.now().isoformat()
                    }
                    # 如果是调试模式，添加调试信息
                    if debug_info:
                        dialogue["debug_info"] = debug_info
                    return [dialogue]
            except (json.JSONDecodeError, Exception) as e:
                print(f"修复JSON失败: {str(e)}，继续尝试其他方法")
            
            # 策略4: 如果JSON解析失败，尝试使用正则表达式提取对话
            dialogues = []
            
            # 尝试匹配多个对话模式
            # 匹配 {"text": "内容"} 格式
            dialogue_pattern = r'\{\s*"text"\s*:\s*"([^"]*(?:\\.[^"]*)*)",?\s*\}'
            matches = re.finditer(dialogue_pattern, cleaned_response)
            
            for match in matches:
                text = match.group(1).replace('\\n', '\n').replace('\\"', '"')
                dialogue = {
                    "text": text,
                    "timestamp": datetime.now().isoformat()
                }
                # 如果是调试模式，添加调试信息
                if debug_info:
                    dialogue["debug_info"] = debug_info
                dialogues.append(dialogue)
            
            if dialogues:
                return dialogues
            
            # 策略5: 尝试提取单个text字段
            # 匹配 "text": "内容" 格式
            text_pattern = r'"text"\s*:\s*"([^"]*(?:\\.[^"]*)*)",?'
            matches = re.finditer(text_pattern, cleaned_response)
            
            for match in matches:
                text = match.group(1).replace('\\n', '\n').replace('\\"', '"')
                dialogue = {
                    "text": text,
                    "timestamp": datetime.now().isoformat()
                }
                # 如果是调试模式，添加调试信息
                if debug_info:
                    dialogue["debug_info"] = debug_info
                dialogues.append(dialogue)
            
            if dialogues:
                return dialogues
            
            # 策略6: 尝试匹配更宽松的模式
            # 匹配双引号之间的任何内容作为可能的对话文本
            loose_pattern = r'"([^"]{20,})"'  # 至少20个字符，避免匹配到短属性
            matches = re.finditer(loose_pattern, cleaned_response)
            
            for match in matches:
                text = match.group(1).replace('\\n', '\n').replace('\\"', '"')
                # 排除明显不是对话内容的文本
                if len(text) > 30 and not text.startswith('{') and not text.startswith('['): 
                    dialogue = {
                        "text": text,
                        "timestamp": datetime.now().isoformat()
                    }
                    # 如果是调试模式，添加调试信息
                    if debug_info:
                        dialogue["debug_info"] = debug_info
                    dialogues.append(dialogue)
            
            if dialogues:
                return dialogues
            
        except Exception as e:
            error_msg = f"解析错误: {str(e)}"
            print(error_msg)
            
            if debug:
                # 调试模式下，返回更详细的错误信息
                debug_data = debug_info.copy() if debug_info else {}
                debug_data.update({
                    "error": str(e),
                    "error_type": type(e).__name__,
                    "raw_response": response,
                    "cleaned_response": cleaned_response if 'cleaned_response' in locals() else response,
                    "json_str": json_str if 'json_str' in locals() else None,
                    "extraction_attempts": [
                        "direct_parse", 
                        "json_extract", 
                        "json_fix", 
                        "regex_dialogue", 
                        "regex_text", 
                        "loose_match"
                    ]
                })
                
                return [{
                    "text": f"解析错误: {str(e)}\n原始响应: {response[:200]}...(截断)",
                    "topic": topic,
                    "timestamp": datetime.now().isoformat(),
                    "debug_info": debug_data
                }]
            
        # 如果所有解析方法都失败，返回空列表
        return []
    
    def _build_generation_prompt(self, background: str, topic: str, tools: List[str], character: str) -> str:
        """构建生成提示词"""
        tools_str = "、".join(tools)
        
        return f"""
生成一段桌宠AI助手的对话。
背景设定：
{background}

AI助手人物设定：
{character}

请基于以上背景设定和人物设定，根据以下话题，创造User不同的提问或者话语，生成1条对话，并翻译这两条对话中User 和 Assistant 的内容为英文、日文、韩文。 请注意AI00不用翻译。
话题：{topic}
AI助手可操作的功能tools有：[{tools_str}]

要求生成格式： json模板，请完全按照模板格式生成
{{
  "dialogues": [
    {{
      "text": "System: 你(Assistant)是一个灵灵生命体，你的名字是银雪，作为天命人(User)的助手，可以聊天也可以使用一些工具。\\n\\nUser: [自然的问题或话语]\\n\\nAssistant: <thinks>\\nmemory: [引用历史对话，50-100字]\\ntools: [调用工具结果，30-50字]\\nsummary: [策略总结，20-30字]\\n</thinks>\\n回复\\n\\n"
    }},
    // ...翻译其他3种语言对话
  ]
}}

**重点注意：**
1. User部分要符合话题设定，可以喊System里设置的名字银雪，也可以不喊。
2. memory为自然引用历史对话，每条记录用[]包括。创造几条符合User输入的内容的主题的对话历史，必定有刚刚的多轮对话内容，还有可能几天前甚至几月前的历史对话摘要。
3. tools为调用的工具返回的内容，格式为[工具名称:工具调用结果]可以有多个工具一起调用，也可以不需要工具调用,显示[无]
4. 请根据memory和tools的内容，总结出summary如何回应User的内容。请简洁的总结，不要超过50字。
5. 回复要温暖、有个性、富有情感，但不要使用字符表情。回复会用TTS发出读音，所以不要有动作的表达，也不要使用[动作]。
6. 回复中，在适合的地方加入[laughter]表示笑声，加入[breath]表示呼吸声。
7. 回复中，适当加入背景设定的元素，但是记住User生活在现实世界，无法到达灵灵宇宙。 

例子： 
{{
  "dialogues": [
    {{
       "text": "System: 你(Assistant)是一个灵灵生命体\\n你的名字是银雪，作为天命人(User)的助手，可以聊天也可以使用一些工具。\\n\\nUser: 今天天气好像很冷，你能帮我看看外面的天气吗？\\n\\nAssistant: <thinks>\\nmemory: [User说：今天天气好像很冷，你能帮我看看外面的天气吗？]\\ntools: [天气API: 今天气温为5°C，有小雪]\\nsummary: 告知用户当前天气情况\\n</thinks>\\n当前温度是5°C，还下着小雪呢。记得多穿点衣服哦，[laughter]灵宝会一直陪着你的[laughter]。\\n\\n"
    }},
       // ...翻译其他3种语言对话
  ]
}}

请直接返回准确的JSON对象，最后一个对象后面不要带","。
"""
    
    async def generate_batch(self, background: str, topic: str, tools: List[str], count: int = 20, temperature: float = 0.8, debug: bool = False, character: str = "") -> List[Dict]:
        """批量生成对话"""
        BATCH_SIZE = 5
        all_results = []
        
        # 分批处理
        total_batches = (count + BATCH_SIZE - 1) // BATCH_SIZE
        for batch_num in range(total_batches):
            remaining = count - batch_num * BATCH_SIZE
            current_batch_size = min(BATCH_SIZE, remaining)
            
            print(f"\n处理第 {batch_num + 1}/{total_batches} 批")
            
            tasks = []
            for _ in range(current_batch_size):
                tasks.append(self.generate_single_dialogue(background, topic, tools, temperature, debug, character))
            
            # 并发执行当前批次
            batch_results = await asyncio.gather(*tasks)
            # 展平结果列表
            flattened_results = []
            for result_list in batch_results:
                if result_list:
                    flattened_results.extend(result_list)
                    self.stats["generated"] += len(result_list)
                else:
                    self.stats["failed"] += 1
            
            all_results.extend(flattened_results)
            
            print(f"当前批次完成：成功生成 {len(flattened_results)} 条")
            
            # 生成进度
            yield {
                "progress": (batch_num + 1) / total_batches,
                "current": len(all_results),
                "total": count,
                "batch_results": flattened_results
            }
        
        self.dialogues = all_results
    
    def save_results(self, filename: str = "dialogues.jsonl", topic: str = "", character: str = ""):
        """保存结果"""
        # 保存训练数据
        with open(filename, "w", encoding="utf-8") as f:
            for d in self.dialogues:
                # 只保存text字段到训练数据
                f.write(json.dumps({"text": d["text"]}, ensure_ascii=False) + "\n")
        
        # 保存完整数据和统计
        full_data = {
            "metadata": {
                "topic": topic,
                "character": character,
                "timestamp": datetime.now().isoformat()
            },
            "dialogues": self.dialogues,
            "stats": self.stats
        }
        
        # 保存调试信息（如果有）
        debug_data = []
        for d in self.dialogues:
            if "debug_info" in d:
                debug_item = {
                    "text": d["text"],
                    "topic": d["topic"],
                    "debug_info": d["debug_info"]
                }
                debug_data.append(debug_item)
        
        # 如果有调试数据，保存到单独的文件
        if debug_data:
            with open(filename.replace(".jsonl", "_debug.json"), "w", encoding="utf-8") as f:
                json.dump(debug_data, f, ensure_ascii=False, indent=2)
        
        with open(filename.replace(".jsonl", "_full.json"), "w", encoding="utf-8") as f:
            json.dump(full_data, f, ensure_ascii=False, indent=2)
        
        debug_msg = f" (包含调试信息)" if debug_data else ""
        return f"生成完成！总计：{len(self.dialogues)}条{debug_msg}"

# ==================== GUI部分 ====================

def create_gui():
    """创建Gradio界面"""
    
    # 全局变量存储生成器实例
    generator = None
    
    def check_api_status():
        """检查API状态"""
        # 检查是否设置了OPENAI_API_KEY环境变量
        api_key = os.getenv("OPENAI_API_KEY")
        if api_key:
            return "✅ 已检测到OPENAI_API_KEY"
        else:
            # 即使没有API密钥，也可以使用默认的无需验证的key
            return "ℹ️ 未检测到OPENAI_API_KEY，将使用默认无需验证的key"
    
    async def generate_dialogues(background, topic, tools_text, count, temperature, filename, debug_mode, character, progress=gr.Progress()):
        """生成对话的异步函数"""
        nonlocal generator
        
        # 使用llama.cpp驱动的qwen3模型，不需要检查API密钥
        # 如果设置了OPENAI_API_KEY环境变量会使用它，否则使用默认的无需验证的key
        
        # 解析工具列表
        tools = [tool.strip() for tool in tools_text.split(",") if tool.strip()]
        if not tools:
            return [], "❌ 错误：请至少输入一个工具", "", gr.update(visible=False), ""
        
        # 创建生成器，只使用openai API接口访问llama.cpp驱动的qwen3模型
        llm_client = LLMClient()
        generator = DialogueGenerator(llm_client)
        
        # 生成对话
        all_dialogues = []
        progress(0, desc="开始生成...")
        
        try:
            async for batch_info in generator.generate_batch(background, topic, tools, count, temperature, debug_mode, character):
                progress(batch_info["progress"], 
                        desc=f"生成中... {batch_info['current']}/{batch_info['total']} (使用 qwen3 模型{' - 调试模式' if debug_mode else ''})")
                all_dialogues.extend(batch_info["batch_results"])
            
            # 保存结果
            # 处理文件名和路径
            if '/' in filename or '\\' in filename:
                # 如果包含路径，分离路径和文件名
                path = Path(filename)
                directory = path.parent
                base_filename = path.stem
            else:
                directory = Path('.')
                base_filename = filename
            
            # 确保文件名安全
            safe_filename = "".join(c for c in base_filename if c.isalnum() or c in ('-', '_', ' ')).rstrip()
            if not safe_filename:
                safe_filename = "dialogues"
            
            # 添加时间戳
            timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
            safe_filename = f"{safe_filename}_{timestamp}"
            
            # 创建目录（如果需要）
            if str(directory) != '.':
                directory.mkdir(parents=True, exist_ok=True)
            
            final_path = directory / f"{safe_filename}.jsonl"
            generator.save_results(str(final_path), topic=topic, character=character)
            
            # 格式化显示结果
            display_results = []
            for i, d in enumerate(all_dialogues[:10], 1):  # 只显示前10条
                display_results.append(f"=== 对话 {i} ===\n{d['text']}\n")
            
            if len(all_dialogues) > 10:
                display_results.append(f"\n... 还有 {len(all_dialogues) - 10} 条对话")
            
            # 提取调试信息（如果开启了调试模式）
            debug_info_text = ""
            if debug_mode:
                debug_items = []
                for d in all_dialogues:
                    if "debug_info" in d:
                        debug_item = {
                            "text": d["text"][:100] + "...",  # 只显示文本的前100个字符
                            "debug_info": d["debug_info"]
                        }
                        debug_items.append(debug_item)
                
                if debug_items:
                    # 格式化调试信息，突出显示请求和响应数据包
                    debug_info = debug_items[0]["debug_info"]  # 获取第一条对话的调试信息
                    formatted_debug = {
                        "timestamp": datetime.now().isoformat(),
                        "request_data_packet": {
                            "url": LLMConfig.PROVIDERS["openai"]["url"],
                            "headers": "Authorization: Bearer sk-***",  # 隐藏实际的API key
                            "method": "POST",
                            "payload": {
                                "model": debug_info.get("payload", {}).get("model", ""),
                                "messages": debug_info.get("payload", {}).get("messages", []),
                                "temperature": debug_info.get("payload", {}).get("temperature", 0),
                                "max_tokens": debug_info.get("payload", {}).get("max_tokens", 0)
                            },
                            "prompt": debug_info.get("prompt", "")
                        },
                        "response_data_packet": {
                            "status": "200 OK",
                            "headers": {
                                "Content-Type": "application/json",
                                "Connection": "keep-alive"
                            },
                            "body": {
                                "id": debug_info.get("full_response", {}).get("id", ""),
                                "object": debug_info.get("full_response", {}).get("object", ""),
                                "created": debug_info.get("full_response", {}).get("created", 0),
                                "model": debug_info.get("full_response", {}).get("model", ""),
                                "choices": debug_info.get("full_response", {}).get("choices", []),
                                "usage": debug_info.get("full_response", {}).get("usage", {})
                            },
                            "content": debug_info.get("full_response", {}).get("choices", [{}])[0].get("message", {}).get("content", "")
                        }
                    }
                    debug_info_text = json.dumps(formatted_debug, indent=2, ensure_ascii=False)
            
            return (
                display_results, 
                f"✅ 生成完成！共生成 {len(all_dialogues)} 条对话\n📁 已保存文件：\n  - {final_path} (训练数据)\n  - {str(final_path).replace('.jsonl', '_full.json')} (完整数据){' - 包含调试信息' if debug_mode else ''}", 
                "\n".join(display_results),
                gr.update(visible=debug_mode),  # 根据调试模式控制调试信息区域的可见性
                debug_info_text
            )
        
        except Exception as e:
            return [], f"❌ 生成失败：{str(e)}", ""
    
    # 创建界面
    with gr.Blocks(title="LLM对话生成器", theme=gr.themes.Soft()) as demo:
        gr.Markdown("# 🤖 LLM对话生成器")
        gr.Markdown("使用LLM API生成桌宠AI助手的训练对话数据")
        gr.Markdown("💡 **提示**: \n- 生成的数据会保存为两个文件：`.jsonl`格式的训练数据和`_full.json`格式的完整数据（包含元信息）\n- 文件名可以包含路径，如：`data/my_dialogues` 或 `./output/test`")
        
        # API状态
        with gr.Row():
            api_status = gr.Textbox(label="API状态", value=check_api_status(), interactive=False)
        
        # 主要内容区域 - 使用两列布局
        with gr.Row():
            # 左侧输入区域
            with gr.Column(scale=1):
                gr.Markdown("### 📝 输入参数")
                with gr.Column():
                    background_input = gr.Textbox(
                        label="背景设定",
                        value=DEFAULT_BACKGROUND,
                        lines=4,  # 减少行数使界面更紧凑
                        placeholder="输入背景设定...",
                        info="支持Markdown格式，描述AI助手的世界观、设定等"
                    )
                    
                    character_input = gr.Textbox(
                        label="AI助手人物设定",
                        value="种类：灵灵生命体\n种族：银鬃狼族\n性别：女性\n外观：银白色狼耳与蓬松狼尾，尾尖泛着淡蓝色数据流光，耳内绒毛呈星空渐变色。\n语言特点：句尾经常自带狼族特有的“，嗷呜~”。喜欢自称本宝宝。\n性格：极端讨好性人格，小M一枚。情感上极度依赖主人，害怕被丢弃，经常强调主人对她的重要性。",
                        lines=4,
                        placeholder="输入AI助手的性格特征、说话风格等...",
                        info="描述AI助手的个性、语气、行为模式等特征"
                    )
                    
                    topic_input = gr.Textbox(
                        label="话题",
                        value="用户想要AI助手帮助操作电脑的某功能",
                        placeholder="输入要生成的话题...",
                        info="定义对话的主题和场景"
                    )
                    
                    tools_input = gr.Textbox(
                        label="可用工具（逗号分隔）",
                        value=", ".join(DEFAULT_TOOLS),
                        placeholder="输入工具列表，用逗号分隔...",
                        info="AI助手可以调用的工具/功能列表"
                    )
                    

                # 示例按钮
                gr.Markdown("### 💡 示例")
                with gr.Column():
                    gr.Examples(
                        examples=[
                            ["用户询问天气情况，AI助手查询并提供建议", "天气API, 日程管理系统"],
                            ["用户需要整理文件，AI助手协助分类和归档", "查找文件, 整理桌面, 打开程序, 文件分类器"],
                            ["用户想学习新技能，AI助手提供个性化指导", "知识图谱API, 学习进度追踪, 创作灵感库, 技能树系统"],
                            ["用户情绪低落，AI助手进行陪伴和鼓励", "情绪分析器, 音乐推荐引擎, 冥想指导系统"],
                            ["用户探索游戏世界，AI助手引导冒险", "维度扫描器, 魔法花园系统, 法则分析仪, 灵子能量扫描器"]
                        ],
                        inputs=[topic_input, tools_input],
                        label="话题示例"
                    )
            
            # 右侧输出区域
            with gr.Column(scale=1):
                gr.Markdown("### 📊 输出结果")
                with gr.Column():
                    with gr.Row():
                        count_input = gr.Slider(
                            label="生成数量",
                            minimum=1,
                            maximum=1000,
                            value=20,
                            step=1,
                            scale=2
                        )
                        temperature_input = gr.Slider(
                            label="创造性程度",
                            minimum=0.1,
                            maximum=1.5,
                            value=0.8,
                            step=0.1,
                            scale=2,
                            info="较低值更保守，较高值更创造性"
                        )
                    
                    with gr.Row():
                        filename_input = gr.Textbox(
                            label="保存文件名",
                            value=f"dialogues_{datetime.now().strftime('%Y%m%d_%H%M%S')}",
                            placeholder="输入文件名或路径/文件名...",
                            scale=3,
                            interactive=False
                        )
                        auto_filename = gr.Checkbox(
                            label="自动生成文件名",
                            value=True,
                            scale=1
                        )
                    with gr.Row():
                        generate_btn = gr.Button("🚀 开始生成", variant="primary", scale=2)
                        debug_mode = gr.Checkbox(
                            label="调试模式",
                            value=False,
                            scale=1
                        )
                    
                    status_output = gr.Textbox(
                        label="状态", 
                        interactive=False
                    )
                    results_output = gr.Textbox(
                        label="生成结果预览", 
                        lines=20, 
                        interactive=False,
                        max_lines=30
                    )
                    debug_info_output = gr.Code(
                        label="调试信息 - 请求和响应数据包", 
                        language="json",
                        lines=25, 
                        interactive=False,
                        visible=False,
                        max_lines=50
                    )
        
        # 事件处理
        def toggle_filename_input(auto_fn):
            """切换文件名输入框的交互状态"""
            return gr.update(interactive=not auto_fn)
        
        auto_filename.change(
            fn=toggle_filename_input,
            inputs=auto_filename,
            outputs=filename_input
        )
        
        def prepare_generation(background, topic, tools_text, count, temperature, filename, auto_fn, debug_mode, character, progress=gr.Progress()):
            """准备生成，如果需要则更新文件名"""
            if auto_fn:
                filename = f"dialogues_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
            
            # 运行异步生成函数（将progress传递给异步函数）
            loop = asyncio.new_event_loop()
            asyncio.set_event_loop(loop)
            
            try:
                # 获取所有返回值，包括调试信息
                dialogues, status, results, debug_visible, debug_info = loop.run_until_complete(
                    generate_dialogues(background, topic, tools_text, count, temperature, filename, debug_mode, character, progress)
                )
                
                # 如果开启了调试模式但没有调试信息，添加提示
                if debug_mode and not debug_info:
                    debug_info = json.dumps({
                        "message": "未能获取调试信息，请重试或检查API连接",
                        "timestamp": datetime.now().isoformat()
                    }, indent=2, ensure_ascii=False)
                
                # 确保调试信息区域在调试模式下可见
                debug_visible = gr.update(visible=debug_mode, value=debug_info if debug_info else "等待生成...")
                
            except Exception as e:
                import traceback
                error_details = traceback.format_exc()
                status = f"❌ 生成过程中出错: {str(e)}"
                results = ""
                debug_visible = gr.update(visible=debug_mode)
                debug_info = json.dumps({
                    "error": str(e),
                    "details": error_details,
                    "timestamp": datetime.now().isoformat()
                }, indent=2, ensure_ascii=False) if debug_mode else ""
            finally:
                loop.close()
            
            return filename, status, results, debug_visible, debug_info
        
        generate_btn.click(
            fn=prepare_generation,
            inputs=[background_input, topic_input, tools_input, count_input, temperature_input, filename_input, auto_filename, debug_mode, character_input],
            outputs=[filename_input, status_output, results_output, debug_info_output, debug_info_output]
        )
        
        # 添加调试模式复选框的事件处理，控制调试信息区域的可见性
        def update_debug_visibility(debug_enabled):
            """更新调试信息区域的可见性和初始内容"""
            if debug_enabled:
                return gr.update(
                    visible=True, 
                    value="调试模式已启用\n\n生成对话时将显示:\n- 发送的请求数据包\n- 接收的响应数据包\n\n点击「开始生成」按钮开始生成对话"
                )
            else:
                return gr.update(visible=False)
        
        debug_mode.change(
            fn=update_debug_visibility,
            inputs=[debug_mode],
            outputs=[debug_info_output]
        )
        
        # 页脚
        gr.Markdown("---")
        gr.Markdown("🔧 **使用提示**: 默认使用llama.cpp驱动的qwen3模型，API地址为http://127.0.0.1:8899/ | 📝 可选设置OPENAI_API_KEY环境变量")
    
    return demo

# ==================== 主程序 ====================

if __name__ == "__main__":
    print("启动LLM对话生成器GUI...")
    print("如需共享链接，请设置 share=True")
    demo = create_gui()
    demo.launch(
        share=False, 
        server_name="0.0.0.0", 
        server_port=7861,  # 修改端口为7861
        show_error=True,
        inbrowser=True  # 自动打开浏览器
    )