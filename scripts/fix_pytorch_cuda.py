#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
PyTorch CUDA 版本冲突修复脚本

此脚本用于解决 PyTorch 和 torchvision CUDA 版本不匹配的问题。
当出现以下错误时使用：
RuntimeError: Detected that PyTorch and torchvision were compiled with different CUDA versions.
"""

import subprocess
import sys
import torch
import torchvision
import platform

def check_cuda_versions():
    """
    检查当前 PyTorch 和 torchvision 的 CUDA 版本
    """
    print("=" * 60)
    print("PyTorch CUDA 版本检查")
    print("=" * 60)
    
    try:
        pytorch_cuda = torch.version.cuda
        torchvision_cuda = torchvision.__version__
        
        print(f"PyTorch 版本: {torch.__version__}")
        print(f"PyTorch CUDA 版本: {pytorch_cuda}")
        print(f"torchvision 版本: {torchvision.__version__}")
        
        # 检查是否有 CUDA 可用
        cuda_available = torch.cuda.is_available()
        print(f"CUDA 可用: {cuda_available}")
        
        if cuda_available:
            print(f"CUDA 设备数量: {torch.cuda.device_count()}")
            print(f"当前 CUDA 设备: {torch.cuda.current_device()}")
            print(f"设备名称: {torch.cuda.get_device_name()}")
        
        return pytorch_cuda, cuda_available
        
    except Exception as e:
        print(f"检查版本时出错: {e}")
        return None, False

def run_command(command):
    """
    执行命令并显示输出
    """
    print(f"\n执行命令: {command}")
    print("-" * 40)
    
    try:
        result = subprocess.run(
            command, 
            shell=True, 
            check=True, 
            capture_output=True, 
            text=True
        )
        print(result.stdout)
        if result.stderr:
            print(f"警告: {result.stderr}")
        return True
    except subprocess.CalledProcessError as e:
        print(f"命令执行失败: {e}")
        print(f"错误输出: {e.stderr}")
        return False

def fix_pytorch_cuda():
    """
    修复 PyTorch CUDA 版本冲突
    """
    print("\n" + "=" * 60)
    print("开始修复 PyTorch CUDA 版本冲突")
    print("=" * 60)
    
    # 检查当前版本
    pytorch_cuda, cuda_available = check_cuda_versions()
    
    print("\n选择修复方案:")
    print("1. 安装 CUDA 11.8 版本 (推荐，如果有 NVIDIA GPU)")
    print("2. 安装 CPU 版本 (如果没有 GPU 或想避免 CUDA 问题)")
    print("3. 取消")
    
    choice = input("\n请选择 (1/2/3): ").strip()
    
    if choice == "1":
        install_cuda_version()
    elif choice == "2":
        install_cpu_version()
    elif choice == "3":
        print("取消修复")
        return
    else:
        print("无效选择")
        return

def install_cuda_version():
    """
    安装 CUDA 版本的 PyTorch
    """
    print("\n安装 CUDA 11.8 版本的 PyTorch...")
    
    commands = [
        "pip uninstall torch torchvision torchaudio -y",
        "pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118"
    ]
    
    for cmd in commands:
        if not run_command(cmd):
            print(f"命令执行失败: {cmd}")
            return False
    
    print("\nCUDA 版本安装完成！")
    verify_installation()
    return True

def install_cpu_version():
    """
    安装 CPU 版本的 PyTorch
    """
    print("\n安装 CPU 版本的 PyTorch...")
    
    commands = [
        "pip uninstall torch torchvision torchaudio -y",
        "pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cpu"
    ]
    
    for cmd in commands:
        if not run_command(cmd):
            print(f"命令执行失败: {cmd}")
            return False
    
    print("\nCPU 版本安装完成！")
    verify_installation()
    return True

def verify_installation():
    """
    验证安装是否成功
    """
    print("\n" + "=" * 60)
    print("验证安装")
    print("=" * 60)
    
    try:
        # 重新导入以获取新版本
        import importlib
        importlib.reload(torch)
        importlib.reload(torchvision)
        
        print(f"新的 PyTorch 版本: {torch.__version__}")
        print(f"新的 torchvision 版本: {torchvision.__version__}")
        print(f"CUDA 可用: {torch.cuda.is_available()}")
        
        # 测试基本功能
        x = torch.randn(2, 3)
        print(f"\n测试张量创建: {x.shape}")
        
        if torch.cuda.is_available():
            x_cuda = x.cuda()
            print(f"CUDA 张量测试: {x_cuda.device}")
        
        print("\n✅ 安装验证成功！")
        
    except Exception as e:
        print(f"❌ 验证失败: {e}")
        print("请重启 Python 环境后再次测试")

def main():
    """
    主函数
    """
    print("PyTorch CUDA 版本冲突修复工具")
    print(f"Python 版本: {sys.version}")
    print(f"操作系统: {platform.system()} {platform.release()}")
    
    try:
        # 首先检查是否存在版本冲突
        pytorch_cuda, cuda_available = check_cuda_versions()
        
        # 询问是否继续修复
        print("\n是否要修复 PyTorch CUDA 版本冲突？")
        confirm = input("继续 (y/n): ").strip().lower()
        
        if confirm in ['y', 'yes', '是']:
            fix_pytorch_cuda()
        else:
            print("取消修复")
            
    except ImportError as e:
        print(f"导入错误: {e}")
        print("请先安装基本的 PyTorch: pip install torch torchvision")
    except Exception as e:
        print(f"程序执行错误: {e}")

if __name__ == "__main__":
    main()