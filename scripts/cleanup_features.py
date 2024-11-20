#!/usr/bin/env python3
import yaml
import os
import shutil
import re

def cleanup_mod_references(file_path, module_name):
    """Remove module references from Rust mod.rs files"""
    if os.path.exists(file_path):
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Remove pub mod declarations
        content = re.sub(rf'pub\s+mod\s+{module_name}\s*;\s*\n?', '', content)
        
        # Remove use statements for the module
        content = re.sub(rf'use\s+(?:crate|super)::{module_name}.*;\s*\n?', '', content)
        
        with open(file_path, 'w') as f:
            f.write(content)

def cleanup_cargo_toml(feature_name):
    """Remove feature-specific dependencies and configurations from Cargo.toml"""
    if os.path.exists('Cargo.toml'):
        with open('Cargo.toml', 'r') as f:
            content = f.read()
        
        # Remove feature-specific dependencies or configurations
        # This is a simplified example - extend based on your needs
        feature_markers = [
            rf'\n\[features\][^\[]*{feature_name}[^\[]*',
            rf'\n{feature_name}-.*= .*\n'
        ]
        
        for marker in feature_markers:
            content = re.sub(marker, '\n', content)
        
        with open('Cargo.toml', 'w') as f:
            f.write(content)

def cleanup_features():
    # Read feature history
    with open('results/feature_history.yaml', 'r') as f:
        history = yaml.safe_load(f)
    
    # Get list of unimplemented features
    unimplemented = [
        feature_name.lower().replace(' ', '-').replace('_', '-')
        for feature_name, details in history['feature_history'].items()
        if not details['implemented']
    ]
    
    # Files to potentially clean up
    cleanup_patterns = {
        'request-batching': [
            ('src/router/batched.rs', 'batched'),
            ('src/main_batched.rs', None)
        ],
        'connection-pooling': [
            ('src/router/pooled.rs', 'pooled'),
            ('src/main_pooled.rs', None)
        ]
    }
    
    # Remove files and clean up references for unimplemented features
    for feature in unimplemented:
        if feature in cleanup_patterns:
            for file_path, module_name in cleanup_patterns[feature]:
                # Remove the file
                if os.path.exists(file_path):
                    print(f"Removing {file_path}")
                    os.remove(file_path)
                
                # Clean up module references
                if module_name:
                    print(f"Cleaning up references to module '{module_name}'")
                    cleanup_mod_references('src/router/mod.rs', module_name)
                
                # Clean up Cargo.toml
                cleanup_cargo_toml(feature)
    
    # Ensure main.rs exists and contains the current implementation
    if not os.path.exists('src/main.rs'):
        implemented_features = [
            feature_name.lower().replace(' ', '-').replace('_', '-')
            for feature_name, details in history['feature_history'].items()
            if details['implemented']
        ]
        
        # If there's a feature-specific main file for the last implemented feature, use it
        if implemented_features:
            last_feature = implemented_features[-1]
            feature_main = f'src/main_{last_feature}.rs'
            if os.path.exists(feature_main):
                print(f"Using {feature_main} as main.rs")
                shutil.copy(feature_main, 'src/main.rs')
                os.remove(feature_main)

if __name__ == '__main__':
    cleanup_features()
