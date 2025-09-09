import { defineUserConfig } from 'vuepress'
import { viteBundler } from '@vuepress/bundler-vite'
import { plumeTheme } from 'vuepress-theme-plume'

export default defineUserConfig({
  bundler: viteBundler({
    viteOptions: {
      css: {
        preprocessorOptions: {
          scss: {
            charset: false
          }
        }
      }
    }
  }),
  
  base: '/rwkv-agent-kit/',
  
  locales: {
    '/': {
      lang: 'zh-CN',
      title: 'RWKV Agent Kit',
      description: '基于RWKV的智能体开发框架 - 具备真正记忆和思考能力的AI智能体'
    },
    '/en/': {
      lang: 'en-US',
      title: 'RWKV Agent Kit',
      description: 'RWKV-based Agent Development Framework - AI Agents with True Memory and Reasoning Capabilities'
    }
  },
  
  head: [
    ['link', { rel: 'icon', href: '/favicon.ico' }],
    ['link', { rel: 'manifest', href: '/manifest.json' }],
    ['meta', { name: 'theme-color', content: '#1e3a8a' }],
    ['meta', { name: 'apple-mobile-web-app-capable', content: 'yes' }],
    ['meta', { name: 'apple-mobile-web-app-status-bar-style', content: 'black-translucent' }],
    ['meta', { name: 'apple-mobile-web-app-title', content: 'RWKV Agent Kit' }],
    ['meta', { name: 'msapplication-TileColor', content: '#1e3a8a' }],
    ['meta', { name: 'viewport', content: 'width=device-width, initial-scale=1.0, viewport-fit=cover' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'RWKV Agent Kit' }],
    ['meta', { property: 'og:description', content: '基于RWKV的智能体开发框架 - 具备真正记忆和思考能力的AI智能体' }],
    ['meta', { property: 'og:image', content: '/images/og-image.png' }],
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.googleapis.com' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' }],
    ['link', { rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=Noto+Sans+SC:wght@300;400;500;600;700&family=JetBrains+Mono:wght@300;400;500;600;700&display=swap' }]
  ],



  theme: plumeTheme({
    // 基础配置
    hostname: 'https://ai00-x.github.io',
    
    // 页脚配置 - 禁用默认页脚
    footer: false,
    
    // 默认启用，仅当 plugins.git 为 true 时生效
    // 此配置在 plume.config.ts 中无效
    contributors: true,
    
    plugins: {
      // 如果您在此处直接声明为 true，则表示开发环境和生产环境都启用该功能
      git: process.env.NODE_ENV === 'production'
    },
    
    // 评论系统配置
    comment: {
      provider: 'Giscus',
      repo: 'Ai00-X/rwkv-agent-kit',
      repoId: 'R_kgDOPU8f4w',
      category: 'Q&A',
      categoryId: 'DIC_kwDOPU8f484CvIib',
      mapping: 'pathname',
      strict: false,
      lazyLoading: true,
      reactionsEnabled: true,
      inputPosition: 'top',
      lang: 'zh-CN'
    },
    
    // 多语言配置
    locales: {
      '/': {
        profile: {
          avatar: '/images/logo.png',
          name: 'RWKV Agent Kit',
          description: '基于RWKV的智能体开发框架',
          location: 'China',
          organization: 'RWKV Agent Kit Team'
        },
        
        navbar: [
          { text: '首页', link: '/' },
          { text: '用户指南', link: '/guide/' },
          { text: 'API 文档', link: '/api/' },
          { text: '高级功能', link: '/advanced/' },
          { text: '示例', link: '/examples/' }
        ],
        
        sidebar: {
          // 首页不显示侧边栏
          '/': false,
          // 其他所有页面显示统一的侧边栏
          '/guide/': [
            {
              text: '📖 用户指南',
              collapsible: true,
              items: [
                { text: '概述', link: '/guide/' },
                { text: '安装指南', link: '/guide/installation' },
                { text: '基础使用', link: '/guide/basic-usage' },
                { text: '高级特性', link: '/guide/advanced-features' },
                { text: '常见问题', link: '/guide/faq' }
              ]
            },
            {
              text: '🔧 配置说明',
              collapsible: true,
              items: [
                { text: '配置指南', link: '/config/' }
              ]
            },
            {
              text: '📚 API 文档',
              collapsible: true,
              items: [
                { text: 'API 概述', link: '/api/' },
                { text: 'RWKV Agent Kit', link: '/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/api/agent-kit' },
                { text: 'Agents', link: '/api/agents' },
                { text: 'Config', link: '/api/config' },
                { text: 'Database', link: '/api/database' },
                { text: 'Memory', link: '/api/memory' },
                { text: 'Tools', link: '/api/tools' },
                { text: 'Types', link: '/api/types' }
              ]
            },
            {
              text: '🚀 高级功能',
              collapsible: true,
              items: [
                { text: '高级功能概述', link: '/advanced/' },
                { text: '自定义智能体', link: '/advanced/custom-agents' },
                { text: '记忆系统', link: '/advanced/memory-system' },
                { text: '工具开发', link: '/advanced/tool-development' }
              ]
            },
            {
              text: '💡 示例代码',
              collapsible: true,
              items: [
                { text: '示例概述', link: '/examples/' }
              ]
            }
          ],
          '/api/': [
            {
              text: '📖 用户指南',
              collapsible: true,
              items: [
                { text: '概述', link: '/guide/' },
                { text: '安装指南', link: '/guide/installation' },
                { text: '基础使用', link: '/guide/basic-usage' },
                { text: '高级特性', link: '/guide/advanced-features' },
                { text: '常见问题', link: '/guide/faq' }
              ]
            },
            {
              text: '🔧 配置说明',
              collapsible: true,
              items: [
                { text: '配置指南', link: '/config/' }
              ]
            },
            {
              text: '📚 API 文档',
              collapsible: true,
              items: [
                { text: 'API 概述', link: '/api/' },
                { text: 'RWKV Agent Kit', link: '/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/api/agent-kit' },
                { text: 'Agents', link: '/api/agents' },
                { text: 'Config', link: '/api/config' },
                { text: 'Database', link: '/api/database' },
                { text: 'Memory', link: '/api/memory' },
                { text: 'Tools', link: '/api/tools' },
                { text: 'Types', link: '/api/types' }
              ]
            },
            {
              text: '🚀 高级功能',
              collapsible: true,
              items: [
                { text: '高级功能概述', link: '/advanced/' },
                { text: '自定义智能体', link: '/advanced/custom-agents' },
                { text: '记忆系统', link: '/advanced/memory-system' },
                { text: '工具开发', link: '/advanced/tool-development' }
              ]
            },
            {
              text: '💡 示例代码',
              collapsible: true,
              items: [
                { text: '示例概述', link: '/examples/' }
              ]
            }
          ],
          '/advanced/': [
            {
              text: '📖 用户指南',
              collapsible: true,
              items: [
                { text: '概述', link: '/guide/' },
                { text: '安装指南', link: '/guide/installation' },
                { text: '基础使用', link: '/guide/basic-usage' },
                { text: '高级特性', link: '/guide/advanced-features' },
                { text: '常见问题', link: '/guide/faq' }
              ]
            },
            {
              text: '🔧 配置说明',
              collapsible: true,
              items: [
                { text: '配置指南', link: '/config/' }
              ]
            },
            {
              text: '📚 API 文档',
              collapsible: true,
              items: [
                { text: 'API 概述', link: '/api/' },
                { text: 'RWKV Agent Kit', link: '/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/api/agent-kit' },
                { text: 'Agents', link: '/api/agents' },
                { text: 'Config', link: '/api/config' },
                { text: 'Database', link: '/api/database' },
                { text: 'Memory', link: '/api/memory' },
                { text: 'Tools', link: '/api/tools' },
                { text: 'Types', link: '/api/types' }
              ]
            },
            {
              text: '🚀 高级功能',
              collapsible: true,
              items: [
                { text: '高级功能概述', link: '/advanced/' },
                { text: '自定义智能体', link: '/advanced/custom-agents' },
                { text: '记忆系统', link: '/advanced/memory-system' },
                { text: '工具开发', link: '/advanced/tool-development' }
              ]
            },
            {
              text: '💡 示例代码',
              collapsible: true,
              items: [
                { text: '示例概述', link: '/examples/' }
              ]
            }
          ],
          '/examples/': [
            {
              text: '📖 用户指南',
              collapsible: true,
              items: [
                { text: '概述', link: '/guide/' },
                { text: '安装指南', link: '/guide/installation' },
                { text: '基础使用', link: '/guide/basic-usage' },
                { text: '高级特性', link: '/guide/advanced-features' },
                { text: '常见问题', link: '/guide/faq' }
              ]
            },
            {
              text: '🔧 配置说明',
              collapsible: true,
              items: [
                { text: '配置指南', link: '/config/' }
              ]
            },
            {
              text: '📚 API 文档',
              collapsible: true,
              items: [
                { text: 'API 概述', link: '/api/' },
                { text: 'RWKV Agent Kit', link: '/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/api/agent-kit' },
                { text: 'Agents', link: '/api/agents' },
                { text: 'Config', link: '/api/config' },
                { text: 'Database', link: '/api/database' },
                { text: 'Memory', link: '/api/memory' },
                { text: 'Tools', link: '/api/tools' },
                { text: 'Types', link: '/api/types' }
              ]
            },
            {
              text: '🚀 高级功能',
              collapsible: true,
              items: [
                { text: '高级功能概述', link: '/advanced/' },
                { text: '自定义智能体', link: '/advanced/custom-agents' },
                { text: '记忆系统', link: '/advanced/memory-system' },
                { text: '工具开发', link: '/advanced/tool-development' }
              ]
            },
            {
              text: '💡 示例代码',
              collapsible: true,
              items: [
                { text: '示例概述', link: '/examples/' }
              ]
            }
          ],
          '/article/': [
            {
              text: '📖 用户指南',
              collapsible: true,
              items: [
                { text: '概述', link: '/guide/' },
                { text: '安装指南', link: '/guide/installation' },
                { text: '基础使用', link: '/guide/basic-usage' },
                { text: '高级特性', link: '/guide/advanced-features' },
                { text: '常见问题', link: '/guide/faq' }
              ]
            },
            {
              text: '🔧 配置说明',
              collapsible: true,
              items: [
                { text: '配置指南', link: '/config/' }
              ]
            },
            {
              text: '📚 API 文档',
              collapsible: true,
              items: [
                { text: 'API 概述', link: '/api/' },
                { text: 'RWKV Agent Kit', link: '/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/api/agent-kit' },
                { text: 'Agents', link: '/api/agents' },
                { text: 'Config', link: '/api/config' },
                { text: 'Database', link: '/api/database' },
                { text: 'Memory', link: '/api/memory' },
                { text: 'Tools', link: '/api/tools' },
                { text: 'Types', link: '/api/types' }
              ]
            },
            {
              text: '🚀 高级功能',
              collapsible: true,
              items: [
                { text: '高级功能概述', link: '/advanced/' },
                { text: '自定义智能体', link: '/advanced/custom-agents' },
                { text: '记忆系统', link: '/advanced/memory-system' },
                { text: '工具开发', link: '/advanced/tool-development' }
              ]
            },
            {
              text: '💡 示例代码',
              collapsible: true,
              items: [
                { text: '示例概述', link: '/examples/' }
              ]
            }
          ],
          '/config/': [
            {
              text: '📖 用户指南',
              collapsible: true,
              items: [
                { text: '概述', link: '/guide/' },
                { text: '安装指南', link: '/guide/installation' },
                { text: '基础使用', link: '/guide/basic-usage' },
                { text: '高级特性', link: '/guide/advanced-features' },
                { text: '常见问题', link: '/guide/faq' }
              ]
            },
            {
              text: '🔧 配置说明',
              collapsible: true,
              items: [
                { text: '配置指南', link: '/config/' }
              ]
            },
            {
              text: '📚 API 文档',
              collapsible: true,
              items: [
                { text: 'API 概述', link: '/api/' },
                { text: 'RWKV Agent Kit', link: '/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/api/agent-kit' },
                { text: 'Agents', link: '/api/agents' },
                { text: 'Config', link: '/api/config' },
                { text: 'Database', link: '/api/database' },
                { text: 'Memory', link: '/api/memory' },
                { text: 'Tools', link: '/api/tools' },
                { text: 'Types', link: '/api/types' }
              ]
            },
            {
              text: '🚀 高级功能',
              collapsible: true,
              items: [
                { text: '高级功能概述', link: '/advanced/' },
                { text: '自定义智能体', link: '/advanced/custom-agents' },
                { text: '记忆系统', link: '/advanced/memory-system' },
                { text: '工具开发', link: '/advanced/tool-development' }
              ]
            },
            {
              text: '💡 示例代码',
              collapsible: true,
              items: [
                { text: '示例概述', link: '/examples/' }
              ]
            }
          ]
        }
      },
      
      '/en/': {
        profile: {
          avatar: '/images/logo.png',
          name: 'RWKV Agent Kit',
          description: 'RWKV-based Agent Development Framework',
          location: 'China',
          organization: 'RWKV Agent Kit Team'
        },
        
        navbar: [
          { text: 'Home', link: '/en/' },
          { text: 'Guide', link: '/en/guide/' },
          { text: 'API Reference', link: '/en/api/' },
          { text: 'Advanced', link: '/en/advanced/' },
          { text: 'Examples', link: '/en/examples/' }
        ],
        
        sidebar: {
          // English homepage doesn't show sidebar
          '/en/': false,
          // All other English pages show unified sidebar
          '/en/guide/': [
            {
              text: '📖 User Guide',
              collapsible: true,
              items: [
                { text: 'Overview', link: '/en/guide/' },
                { text: 'Installation', link: '/en/guide/installation' },
                { text: 'Basic Usage', link: '/en/guide/basic-usage' },
                { text: 'Advanced Features', link: '/en/guide/advanced-features' },
                { text: 'FAQ', link: '/en/guide/faq' }
              ]
            },
            {
              text: '🔧 Configuration',
              collapsible: true,
              items: [
                { text: 'Configuration Guide', link: '/en/config/' }
              ]
            },
            {
              text: '📚 API Reference',
              collapsible: true,
              items: [
                { text: 'API Overview', link: '/en/api/' },
                { text: 'RWKV Agent Kit', link: '/en/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/en/api/agent-kit' },
                { text: 'Agents', link: '/en/api/agents' },
                { text: 'Config', link: '/en/api/config' },
                { text: 'Database', link: '/en/api/database' },
                { text: 'Memory', link: '/en/api/memory' },
                { text: 'Tools', link: '/en/api/tools' },
                { text: 'Types', link: '/en/api/types' }
              ]
            },
            {
              text: '🚀 Advanced Features',
              collapsible: true,
              items: [
                { text: 'Advanced Overview', link: '/en/advanced/' },
                { text: 'Custom Agents', link: '/en/advanced/custom-agents' },
                { text: 'Memory System', link: '/en/advanced/memory-system' },
                { text: 'Tool Development', link: '/en/advanced/tool-development' }
              ]
            },
            {
              text: '💡 Examples',
              collapsible: true,
              items: [
                { text: 'Examples Overview', link: '/en/examples/' }
              ]
            }
          ],
          '/en/api/': [
            {
              text: '📖 User Guide',
              collapsible: true,
              items: [
                { text: 'Overview', link: '/en/guide/' },
                { text: 'Installation', link: '/en/guide/installation' },
                { text: 'Basic Usage', link: '/en/guide/basic-usage' },
                { text: 'Advanced Features', link: '/en/guide/advanced-features' },
                { text: 'FAQ', link: '/en/guide/faq' }
              ]
            },
            {
              text: '🔧 Configuration',
              collapsible: true,
              items: [
                { text: 'Configuration Guide', link: '/en/config/' }
              ]
            },
            {
              text: '📚 API Reference',
              collapsible: true,
              items: [
                { text: 'API Overview', link: '/en/api/' },
                { text: 'RWKV Agent Kit', link: '/en/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/en/api/agent-kit' },
                { text: 'Agents', link: '/en/api/agents' },
                { text: 'Config', link: '/en/api/config' },
                { text: 'Database', link: '/en/api/database' },
                { text: 'Memory', link: '/en/api/memory' },
                { text: 'Tools', link: '/en/api/tools' },
                { text: 'Types', link: '/en/api/types' }
              ]
            },
            {
              text: '🚀 Advanced Features',
              collapsible: true,
              items: [
                { text: 'Advanced Overview', link: '/en/advanced/' },
                { text: 'Custom Agents', link: '/en/advanced/custom-agents' },
                { text: 'Memory System', link: '/en/advanced/memory-system' },
                { text: 'Tool Development', link: '/en/advanced/tool-development' }
              ]
            },
            {
              text: '💡 Examples',
              collapsible: true,
              items: [
                { text: 'Examples Overview', link: '/en/examples/' }
              ]
            }
          ],
          '/en/advanced/': [
            {
              text: '📖 User Guide',
              collapsible: true,
              items: [
                { text: 'Overview', link: '/en/guide/' },
                { text: 'Installation', link: '/en/guide/installation' },
                { text: 'Basic Usage', link: '/en/guide/basic-usage' },
                { text: 'Advanced Features', link: '/en/guide/advanced-features' },
                { text: 'FAQ', link: '/en/guide/faq' }
              ]
            },
            {
              text: '🔧 Configuration',
              collapsible: true,
              items: [
                { text: 'Configuration Guide', link: '/en/config/' }
              ]
            },
            {
              text: '📚 API Reference',
              collapsible: true,
              items: [
                { text: 'API Overview', link: '/en/api/' },
                { text: 'RWKV Agent Kit', link: '/en/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/en/api/agent-kit' },
                { text: 'Agents', link: '/en/api/agents' },
                { text: 'Config', link: '/en/api/config' },
                { text: 'Database', link: '/en/api/database' },
                { text: 'Memory', link: '/en/api/memory' },
                { text: 'Tools', link: '/en/api/tools' },
                { text: 'Types', link: '/en/api/types' }
              ]
            },
            {
              text: '🚀 Advanced Features',
              collapsible: true,
              items: [
                { text: 'Advanced Overview', link: '/en/advanced/' },
                { text: 'Custom Agents', link: '/en/advanced/custom-agents' },
                { text: 'Memory System', link: '/en/advanced/memory-system' },
                { text: 'Tool Development', link: '/en/advanced/tool-development' }
              ]
            },
            {
              text: '💡 Examples',
              collapsible: true,
              items: [
                { text: 'Examples Overview', link: '/en/examples/' }
              ]
            }
          ],
          '/en/examples/': [
            {
              text: '📖 User Guide',
              collapsible: true,
              items: [
                { text: 'Overview', link: '/en/guide/' },
                { text: 'Installation', link: '/en/guide/installation' },
                { text: 'Basic Usage', link: '/en/guide/basic-usage' },
                { text: 'Advanced Features', link: '/en/guide/advanced-features' },
                { text: 'FAQ', link: '/en/guide/faq' }
              ]
            },
            {
              text: '🔧 Configuration',
              collapsible: true,
              items: [
                { text: 'Configuration Guide', link: '/en/config/' }
              ]
            },
            {
              text: '📚 API Reference',
              collapsible: true,
              items: [
                { text: 'API Overview', link: '/en/api/' },
                { text: 'RWKV Agent Kit', link: '/en/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/en/api/agent-kit' },
                { text: 'Agents', link: '/en/api/agents' },
                { text: 'Config', link: '/en/api/config' },
                { text: 'Database', link: '/en/api/database' },
                { text: 'Memory', link: '/en/api/memory' },
                { text: 'Tools', link: '/en/api/tools' },
                { text: 'Types', link: '/en/api/types' }
              ]
            },
            {
              text: '🚀 Advanced Features',
              collapsible: true,
              items: [
                { text: 'Advanced Overview', link: '/en/advanced/' },
                { text: 'Custom Agents', link: '/en/advanced/custom-agents' },
                { text: 'Memory System', link: '/en/advanced/memory-system' },
                { text: 'Tool Development', link: '/en/advanced/tool-development' }
              ]
            },
            {
              text: '💡 Examples',
              collapsible: true,
              items: [
                { text: 'Examples Overview', link: '/en/examples/' }
              ]
            }
          ],
          '/en/article/': [
            {
              text: '📖 User Guide',
              collapsible: true,
              items: [
                { text: 'Overview', link: '/en/guide/' },
                { text: 'Installation', link: '/en/guide/installation' },
                { text: 'Basic Usage', link: '/en/guide/basic-usage' },
                { text: 'Advanced Features', link: '/en/guide/advanced-features' },
                { text: 'FAQ', link: '/en/guide/faq' }
              ]
            },
            {
              text: '🔧 Configuration',
              collapsible: true,
              items: [
                { text: 'Configuration Guide', link: '/en/config/' }
              ]
            },
            {
              text: '📚 API Reference',
              collapsible: true,
              items: [
                { text: 'API Overview', link: '/en/api/' },
                { text: 'RWKV Agent Kit', link: '/en/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/en/api/agent-kit' },
                { text: 'Agents', link: '/en/api/agents' },
                { text: 'Config', link: '/en/api/config' },
                { text: 'Database', link: '/en/api/database' },
                { text: 'Memory', link: '/en/api/memory' },
                { text: 'Tools', link: '/en/api/tools' },
                { text: 'Types', link: '/en/api/types' }
              ]
            },
            {
              text: '🚀 Advanced Features',
              collapsible: true,
              items: [
                { text: 'Advanced Overview', link: '/en/advanced/' },
                { text: 'Custom Agents', link: '/en/advanced/custom-agents' },
                { text: 'Memory System', link: '/en/advanced/memory-system' },
                { text: 'Tool Development', link: '/en/advanced/tool-development' }
              ]
            },
            {
              text: '💡 Examples',
              collapsible: true,
              items: [
                { text: 'Examples Overview', link: '/en/examples/' }
              ]
            }
          ],
          '/en/config/': [
            {
              text: '📖 User Guide',
              collapsible: true,
              items: [
                { text: 'Overview', link: '/en/guide/' },
                { text: 'Installation', link: '/en/guide/installation' },
                { text: 'Basic Usage', link: '/en/guide/basic-usage' },
                { text: 'Advanced Features', link: '/en/guide/advanced-features' },
                { text: 'FAQ', link: '/en/guide/faq' }
              ]
            },
            {
              text: '🔧 Configuration',
              collapsible: true,
              items: [
                { text: 'Configuration Guide', link: '/en/config/' }
              ]
            },
            {
              text: '📚 API Reference',
              collapsible: true,
              items: [
                { text: 'API Overview', link: '/en/api/' },
                { text: 'RWKV Agent Kit', link: '/en/api/rwkv-agent-kit' },
                { text: 'Agent Kit', link: '/en/api/agent-kit' },
                { text: 'Agents', link: '/en/api/agents' },
                { text: 'Config', link: '/en/api/config' },
                { text: 'Database', link: '/en/api/database' },
                { text: 'Memory', link: '/en/api/memory' },
                { text: 'Tools', link: '/en/api/tools' },
                { text: 'Types', link: '/en/api/types' }
              ]
            },
            {
              text: '🚀 Advanced Features',
              collapsible: true,
              items: [
                { text: 'Advanced Overview', link: '/en/advanced/' },
                { text: 'Custom Agents', link: '/en/advanced/custom-agents' },
                { text: 'Memory System', link: '/en/advanced/memory-system' },
                { text: 'Tool Development', link: '/en/advanced/tool-development' }
              ]
            },
            {
              text: '💡 Examples',
              collapsible: true,
              items: [
                { text: 'Examples Overview', link: '/en/examples/' }
              ]
            }
          ]
        }
      }
    }
  })
})