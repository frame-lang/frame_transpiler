# VS Code Extension Integration with Frame Transpiler

**Version:** v0.79.0  
**Date:** October 1, 2025  
**Purpose:** Guide for VS Code extension team to integrate with Frame transpiler validation infrastructure

## Overview

The Frame transpiler now provides a comprehensive **source of truth** validation system for source map quality assessment. The VS Code extension should rely on these transpiler-provided tools rather than implementing its own validation logic.

## Core Integration Principle

**❌ DON'T**: Implement custom source map validation in the extension  
**✅ DO**: Call transpiler validation tools and interpret the results

This ensures:
- ✅ Consistent quality assessment between transpiler team and extension
- ✅ Single source of truth prevents conflicting reports
- ✅ Quality standards evolve with transpiler improvements
- ✅ Bug detection patterns maintained by transpiler experts

## Validation Tools Available

### 1. Single File Validation
**Use Case**: Real-time validation during debugging sessions

```bash
# Command
python3 tools/test_framework_integration.py --file path/to/file.frm --mode json

# Example Output (JSON)
{
  "file": "test_debug_entry.frm",
  "passed": true,
  "analysis": {
    "quality_classification": "EXCELLENT",
    "executable_coverage": 100.0,
    "duplicates": 2,
    "bug27_patterns": 2,
    "recommendations": []
  },
  "timestamp": "2025-10-01T15:26:08-07:00",
  "transpiler_version": "framec 0.79.0"
}
```

### 2. Overall Quality Status
**Use Case**: Extension status bar, settings UI

```bash
# Command  
python3 tools/test_framework_integration.py --mode vscode-status

# Example Output (JSON)
{
  "status": "ready",
  "source_of_truth": true,
  "validation_tool": "Frame Transpiler Source Map Validator",
  "quality": {
    "classification": "GOOD",
    "pass_rate": 76.8,
    "coverage": 84.2,
    "duplicates": 683
  },
  "standards": {
    "minimum_executable_coverage": 95,
    "maximum_acceptable_duplicates": 5
  },
  "last_validation": "2025-10-01T15:26:08-07:00",
  "transpiler_version": "framec 0.79.0"
}
```

### 3. Test Suite Validation
**Use Case**: Extension settings, workspace analysis

```bash
# Command
python3 tools/source_map_test_integration.py --test-dir path/to/tests --report /tmp/quality.json

# Generates comprehensive quality report for workspace
```

## Extension Integration Architecture

### Recommended TypeScript Integration

```typescript
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export class FrameTranspilerValidation {
    private transpilerPath: string;
    private toolsPath: string;

    constructor(transpilerPath: string) {
        this.transpilerPath = transpilerPath;
        this.toolsPath = `${transpilerPath}/tools`;
    }

    /**
     * Validate source map quality for a single Frame file
     */
    async validateFile(frameFilePath: string): Promise<FileValidation> {
        try {
            const cmd = `python3 ${this.toolsPath}/test_framework_integration.py --file "${frameFilePath}" --mode json`;
            const { stdout } = await execAsync(cmd);
            return JSON.parse(stdout) as FileValidation;
        } catch (error) {
            throw new Error(`Validation failed: ${error.message}`);
        }
    }

    /**
     * Get overall transpiler source map quality status
     */
    async getQualityStatus(): Promise<QualityStatus> {
        try {
            const cmd = `python3 ${this.toolsPath}/test_framework_integration.py --mode vscode-status`;
            const { stdout } = await execAsync(cmd);
            return JSON.parse(stdout) as QualityStatus;
        } catch (error) {
            throw new Error(`Status check failed: ${error.message}`);
        }
    }

    /**
     * Validate entire workspace/project
     */
    async validateWorkspace(testDir: string): Promise<WorkspaceValidation> {
        try {
            const reportPath = `/tmp/frame_workspace_quality_${Date.now()}.json`;
            const cmd = `python3 ${this.toolsPath}/source_map_test_integration.py --test-dir "${testDir}" --report "${reportPath}"`;
            
            await execAsync(cmd);
            
            const fs = require('fs');
            const report = JSON.parse(fs.readFileSync(reportPath, 'utf8'));
            fs.unlinkSync(reportPath); // Clean up
            
            return report as WorkspaceValidation;
        } catch (error) {
            throw new Error(`Workspace validation failed: ${error.message}`);
        }
    }
}

// Type definitions
interface FileValidation {
    file: string;
    passed: boolean;
    analysis: {
        quality_classification: 'EXCELLENT' | 'GOOD' | 'FAIR' | 'POOR';
        executable_coverage: number;
        duplicates: number;
        bug27_patterns: number;
        recommendations: string[];
    };
    timestamp: string;
    transpiler_version: string;
}

interface QualityStatus {
    status: string;
    source_of_truth: boolean;
    validation_tool: string;
    quality: {
        classification: 'EXCELLENT' | 'GOOD' | 'FAIR' | 'POOR';
        pass_rate: number;
        coverage: number;
        duplicates: number;
    };
    standards: {
        minimum_executable_coverage: number;
        maximum_acceptable_duplicates: number;
    };
    last_validation: string;
    transpiler_version: string;
}

interface WorkspaceValidation {
    transpiler_version: string;
    total_files: number;
    passed_files: number;
    pass_rate: number;
    quality_classification: 'EXCELLENT' | 'GOOD' | 'FAIR' | 'POOR';
    total_duplicates: number;
    average_coverage: number;
    failed_files: Array<{
        file: string;
        summary: string;
    }>;
    recommendations: string[];
}
```

## Extension AI Integration

### AI Assistant for Source Map Quality

The extension AI should use the validation results to provide intelligent debugging guidance:

```typescript
export class FrameDebuggingAI {
    private validator: FrameTranspilerValidation;

    constructor(validator: FrameTranspilerValidation) {
        this.validator = validator;
    }

    /**
     * Assess expected debugging experience for a Frame file
     */
    async assessDebuggingExperience(frameFile: string): Promise<DebuggingAssessment> {
        const validation = await this.validator.validateFile(frameFile);
        
        if (validation.analysis.quality_classification === 'EXCELLENT') {
            return {
                level: 'excellent',
                message: 'Perfect debugging experience expected',
                recommendations: [
                    'Set breakpoints anywhere in the code',
                    'Step-through debugging will be highly accurate',
                    'All Frame lines properly mapped to Python execution'
                ],
                confidence: 0.95
            };
        }
        
        if (validation.analysis.quality_classification === 'GOOD') {
            return {
                level: 'good',
                message: 'Reliable debugging with minor limitations',
                recommendations: [
                    'Breakpoints work well for main logic',
                    'Some complex expressions may have step-through quirks',
                    'Overall debugging experience is solid'
                ],
                confidence: 0.85
            };
        }
        
        if (validation.analysis.duplicates > 10) {
            return {
                level: 'warning',
                message: 'Multiple duplicate mappings detected',
                recommendations: [
                    'Debugger may stop multiple times on same Frame line',
                    'Focus breakpoints on main logic rather than complex expressions',
                    'Consider updating to latest transpiler version'
                ],
                confidence: 0.75,
                issues: [`${validation.analysis.duplicates} duplicate mappings found`]
            };
        }
        
        if (validation.analysis.quality_classification === 'POOR') {
            return {
                level: 'poor',
                message: 'Significant debugging limitations expected',
                recommendations: [
                    'Limited breakpoint accuracy',
                    'Consider updating transpiler for better source maps',
                    'Use print statements for complex debugging scenarios'
                ],
                confidence: 0.60,
                issues: [
                    `Coverage: ${validation.analysis.executable_coverage}%`,
                    `${validation.analysis.duplicates} duplicate mappings`
                ]
            };
        }
        
        return {
            level: 'unknown',
            message: 'Unable to assess debugging quality',
            recommendations: ['Try updating the transpiler'],
            confidence: 0.30
        };
    }

    /**
     * Suggest transpiler actions based on validation results
     */
    async suggestTranspilerActions(frameFile: string): Promise<TranspilerActionSuggestion> {
        const validation = await this.validator.validateFile(frameFile);
        const status = await this.validator.getQualityStatus();
        
        if (validation.analysis.quality_classification === 'POOR') {
            return {
                action: 'update_transpiler',
                priority: 'high',
                reason: 'Source map quality below debugging standards',
                expected_improvement: 'Better breakpoint accuracy and step-through behavior',
                current_version: validation.transpiler_version
            };
        }
        
        if (validation.analysis.bug27_patterns > 5) {
            return {
                action: 'report_bug',
                priority: 'medium',
                reason: `${validation.analysis.bug27_patterns} Bug #27 patterns detected`,
                expected_improvement: 'Cleaner step-through without duplicate stops',
                bug_details: 'Duplicate mappings for event handlers and state transitions'
            };
        }
        
        if (status.quality.classification === 'EXCELLENT') {
            return {
                action: 'no_action_needed',
                priority: 'none',
                reason: 'Source map quality is excellent',
                message: 'Optimal debugging experience available'
            };
        }
        
        return {
            action: 'monitor',
            priority: 'low',
            reason: 'Source map quality is acceptable',
            message: 'Consider checking for transpiler updates periodically'
        };
    }
}

// Additional type definitions
interface DebuggingAssessment {
    level: 'excellent' | 'good' | 'warning' | 'poor' | 'unknown';
    message: string;
    recommendations: string[];
    confidence: number;
    issues?: string[];
}

interface TranspilerActionSuggestion {
    action: 'update_transpiler' | 'report_bug' | 'no_action_needed' | 'monitor';
    priority: 'high' | 'medium' | 'low' | 'none';
    reason: string;
    expected_improvement?: string;
    current_version?: string;
    bug_details?: string;
    message?: string;
}
```

## Extension UI Integration

### Status Bar Integration

```typescript
// Show source map quality in status bar
export class FrameStatusBar {
    private statusBarItem: vscode.StatusBarItem;
    private validator: FrameTranspilerValidation;

    constructor(validator: FrameTranspilerValidation) {
        this.validator = validator;
        this.statusBarItem = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Right, 
            100
        );
    }

    async updateStatus() {
        try {
            const status = await this.validator.getQualityStatus();
            
            const icon = this.getQualityIcon(status.quality.classification);
            this.statusBarItem.text = `$(${icon}) Frame: ${status.quality.classification}`;
            this.statusBarItem.tooltip = `Source Map Quality: ${status.quality.classification}\\n` +
                                       `Pass Rate: ${status.quality.pass_rate.toFixed(1)}%\\n` +
                                       `Coverage: ${status.quality.coverage.toFixed(1)}%\\n` +
                                       `Transpiler: ${status.transpiler_version}`;
            
            this.statusBarItem.show();
        } catch (error) {
            this.statusBarItem.text = `$(alert) Frame: Unknown`;
            this.statusBarItem.tooltip = `Source map validation failed: ${error.message}`;
        }
    }

    private getQualityIcon(classification: string): string {
        switch (classification) {
            case 'EXCELLENT': return 'check-all';
            case 'GOOD': return 'check';
            case 'FAIR': return 'warning';
            case 'POOR': return 'error';
            default: return 'question';
        }
    }
}
```

### Debug Session Integration

```typescript
// Integrate validation with debug sessions
export class FrameDebugSessionEnhancer {
    private validator: FrameTranspilerValidation;
    private ai: FrameDebuggingAI;

    async onDebugSessionStart(session: vscode.DebugSession, frameFile: string) {
        // Validate source maps before debugging starts
        const assessment = await this.ai.assessDebuggingExperience(frameFile);
        
        if (assessment.level === 'poor' || assessment.level === 'warning') {
            const message = `Debugging Quality: ${assessment.message}`;
            const action = await vscode.window.showWarningMessage(
                message,
                'Continue Anyway',
                'Learn More',
                'Update Transpiler'
            );
            
            if (action === 'Learn More') {
                this.showDebuggingGuidance(assessment);
            } else if (action === 'Update Transpiler') {
                this.suggestTranspilerUpdate();
            }
        }
    }

    private async showDebuggingGuidance(assessment: DebuggingAssessment) {
        const panel = vscode.window.createWebviewPanel(
            'frameDebuggingGuidance',
            'Frame Debugging Guidance',
            vscode.ViewColumn.Beside,
            {}
        );

        panel.webview.html = `
            <h2>Debugging Experience: ${assessment.level.toUpperCase()}</h2>
            <p>${assessment.message}</p>
            <h3>Recommendations:</h3>
            <ul>
                ${assessment.recommendations.map(rec => `<li>${rec}</li>`).join('')}
            </ul>
            ${assessment.issues ? `
                <h3>Issues Detected:</h3>
                <ul>
                    ${assessment.issues.map(issue => `<li>${issue}</li>`).join('')}
                </ul>
            ` : ''}
            <p><strong>Confidence:</strong> ${(assessment.confidence * 100).toFixed(0)}%</p>
        `;
    }
}
```

## Quality Thresholds

The extension should use these transpiler-defined quality standards:

| Classification | Executable Coverage | Max Duplicates | Extension Behavior |
|----------------|-------------------|----------------|-------------------|
| **EXCELLENT**  | ≥95%              | ≤2             | 🟢 Green status, no warnings |
| **GOOD**       | ≥90%              | ≤5             | 🟢 Green status, minor notices |
| **FAIR**       | ≥80%              | ≤10            | 🟡 Yellow status, debugging tips |
| **POOR**       | <80%              | >10            | 🔴 Red status, strong recommendations |

## Installation Requirements

The extension should check for required dependencies:

1. **Python 3.x**: Required for validation tools
2. **Frame Transpiler**: Minimum version with validation support (v0.79.0+)
3. **Tools Directory**: Validation scripts must be available at transpiler installation

```typescript
// Dependency checking
export class FrameExtensionSetup {
    async verifyRequirements(): Promise<SetupStatus> {
        const checks = {
            python: await this.checkPython(),
            transpiler: await this.checkTranspiler(),
            validationTools: await this.checkValidationTools()
        };

        return {
            ready: Object.values(checks).every(check => check.available),
            checks
        };
    }

    private async checkPython(): Promise<DependencyCheck> {
        try {
            const { stdout } = await execAsync('python3 --version');
            return {
                available: true,
                version: stdout.trim(),
                message: 'Python 3 available'
            };
        } catch {
            return {
                available: false,
                message: 'Python 3 required for source map validation'
            };
        }
    }

    private async checkTranspiler(): Promise<DependencyCheck> {
        try {
            const { stdout } = await execAsync('framec --version');
            const version = stdout.trim();
            const versionMatch = version.match(/framec (\\d+)\\.(\\d+)\\.(\\d+)/);
            
            if (versionMatch) {
                const [, major, minor, patch] = versionMatch.map(Number);
                const hasValidation = major > 0 || minor >= 79;
                
                return {
                    available: hasValidation,
                    version,
                    message: hasValidation 
                        ? 'Frame transpiler with validation support'
                        : 'Frame transpiler found but validation requires v0.79.0+'
                };
            }
            
            return {
                available: false,
                message: 'Unable to determine transpiler version'
            };
        } catch {
            return {
                available: false,
                message: 'Frame transpiler not found in PATH'
            };
        }
    }

    private async checkValidationTools(): Promise<DependencyCheck> {
        try {
            const toolPath = 'tools/test_framework_integration.py'; // Relative to transpiler
            await execAsync(`python3 ${toolPath} --help`);
            return {
                available: true,
                message: 'Validation tools available'
            };
        } catch {
            return {
                available: false,
                message: 'Validation tools not found - check transpiler installation'
            };
        }
    }
}
```

## Error Handling

The extension should gracefully handle validation failures:

```typescript
export class FrameValidationErrorHandler {
    handleValidationError(error: Error, context: string): ValidationFallback {
        console.error(`Frame validation error in ${context}:`, error);

        // Provide fallback behavior
        return {
            quality: 'UNKNOWN',
            message: 'Unable to validate source maps',
            recommendations: [
                'Check transpiler installation',
                'Verify Python 3 availability',
                'Try restarting VS Code'
            ],
            fallbackBehavior: 'assume_basic_debugging'
        };
    }
}
```

## Summary

The VS Code extension should:

1. ✅ **Use transpiler validation tools** as the authoritative source
2. ✅ **Interpret results** to provide user-friendly guidance
3. ✅ **Integrate with debugging sessions** to set appropriate expectations
4. ✅ **Display quality status** in the UI (status bar, notifications)
5. ✅ **Provide AI-powered recommendations** based on validation results
6. ✅ **Handle errors gracefully** when validation tools aren't available

This approach ensures the extension provides accurate, consistent debugging guidance that evolves with transpiler improvements.