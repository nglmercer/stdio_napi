/**
 * Terminal TTY Example
 * 
 * Comprehensive demonstration of terminal/TTY features.
 * Run: bun run examples/terminal/info.ts
 */

import {
  getTerminalSize,
  getTerminalInfo,
  setTerminalTitle,
  clearScreen,
  moveCursor,
  showCursor,
  hideCursor,
  setCursorShape,
  CursorShape,
  enableRawMode,
  disableRawMode,
  isStdoutTty,
  isStdinTty,
  isStderrTty,
  moveCursorUp,
  moveCursorDown,
  moveCursorLeft,
  moveCursorRight,
  moveCursorToColumn,
  moveCursorNextLine,
  moveCursorPreviousLine,
  clearCurrentLine,
  clearUntilNewline,
  clearFromCursor,
  saveCursorPosition,
  restoreCursorPosition,
  enterAlternateScreen,
  leaveAlternateScreen,
  scrollUp,
  scrollDown,
  printInfo,
  printSuccess,
  printWarning,
  printError,
  printProgress,
  getSpinnerFrame,
  readLineInteractive,
} from "../../index.js";

// Color codes for demo
const colors = {
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
  reset: '\x1b[0m',
  bold: '\x1b[1m',
};

function printHeader(text: string) {
  console.log(`\n${colors.bold}${colors.cyan}${text}${colors.reset}`);
}

function printSection(text: string) {
  console.log(`\n${colors.bold}${colors.yellow}${text}${colors.reset}`);
}

function demoBasicInfo() {
  printHeader("═══ Terminal Basic Info ═══");
  
  // Check TTY status
  console.log(`\n${colors.cyan}TTY Status:${colors.reset}`);
  console.log(`  stdout is TTY: ${isStdoutTty() ? colors.green + 'Yes' + colors.reset : colors.red + 'No' + colors.reset}`);
  console.log(`  stdin is TTY:  ${isStdinTty() ? colors.green + 'Yes' + colors.reset : colors.red + 'No' + colors.reset}`);
  console.log(`  stderr is TTY: ${isStderrTty() ? colors.green + 'Yes' + colors.reset : colors.red + 'No' + colors.reset}`);
  
  // Get terminal size
  try {
    const size = getTerminalSize();
    console.log(`\n${colors.cyan}Terminal Size:${colors.reset}`);
    console.log(`  Columns: ${colors.green}${size.columns}${colors.reset}`);
    console.log(`  Rows:    ${colors.green}${size.rows}${colors.reset}`);
  } catch (e) {
    printError(`Failed to get terminal size: ${e}`);
  }
  
  // Get terminal info
  const info = getTerminalInfo();
  console.log(`\n${colors.cyan}Terminal Info:${colors.reset}`);
  console.log(`  Type:        ${colors.green}${info.terminalType}${colors.reset}`);
  console.log(`  Color Support: ${colors.green}${info.colorSupport}${colors.reset}`);
}

function demoCursorMovement() {
  printSection("═══ Cursor Movement Demo ═══");
  
  console.log("Moving cursor around...");
  
  // Move to position (10, 5)
  moveCursor(10, 5);
  console.log("  Moved to (10, 5)");
  
  // Move up
  moveCursorUp(2);
  console.log("  Moved up 2 rows");
  
  // Move down
  moveCursorDown(3);
  console.log("  Moved down 3 rows");
  
  // Move right
  moveCursorRight(5);
  console.log("  Moved right 5 columns");
  
  // Move left
  moveCursorLeft(5);
  console.log("  Moved left 5 columns");
  
  // Move to column
  moveCursorToColumn(1);
  console.log("  Moved to column 1");
  
  // Next/previous line
  moveCursorNextLine(1);
  console.log("  Moved to next line");
  moveCursorPreviousLine(1);
  console.log("  Moved to previous line");
  
  // Save and restore position
  saveCursorPosition();
  moveCursor(20, 10);
  console.log("  Saved position and moved to (20, 10)");
  restoreCursorPosition();
  console.log("  Restored to saved position");
}

function demoScreenClearing() {
  printSection("═══ Screen Clearing Demo ═══");
  
  console.log("Various ways to clear the screen:");
  console.log("  1. clearCurrentLine() - clears current line");
  clearCurrentLine();
  
  console.log("  2. clearUntilNewline() - clears to end of line");
  clearUntilNewline();
  
  console.log("  3. clearFromCursor() - clears from cursor up");
  clearFromCursor();
  
  console.log("\nNote: full clearScreen() clears entire screen");
}

function demoAlternateScreen() {
  printSection("═══ Alternate Screen Demo ═══");
  
  console.log("Entering alternate screen...");
  enterAlternateScreen();
  
  console.log(`${colors.green}Now in alternate screen!${colors.reset}`);
  console.log("This screen is separate from the main terminal buffer.");
  console.log("Press Enter to leave...");
  
  leaveAlternateScreen();
  
  console.log(`${colors.yellow}Back to main screen!${colors.reset}`);
}

function demoCursorVisibility() {
  printSection("═══ Cursor Visibility Demo ═══");
  
  console.log("Hiding cursor...");
  hideCursor();
  console.log("  Cursor hidden");
  
  console.log("Waiting 2 seconds...");
  setTimeout(() => {
    showCursor();
    console.log("  Cursor shown again");
  }, 2000);
}

function demoCursorShapes() {
  printSection("═══ Cursor Shape Demo ═══");
  
  const shapes: Array<{ shape: CursorShape; name: string }> = [
    { shape: CursorShape.Block, name: "Block" },
    { shape: CursorShape.BlinkingBlock, name: "Blinking Block" },
    { shape: CursorShape.Underline, name: "Underline" },
    { shape: CursorShape.BlinkingUnderline, name: "Blinking Underline" },
    { shape: CursorShape.Bar, name: "Bar (I-beam)" },
    { shape: CursorShape.BlinkingBar, name: "Blinking Bar" },
  ];
  
  console.log("Cursor shapes (changing every 1 second):");
  for (const { shape, name } of shapes) {
    setCursorShape(shape);
    console.log(`  ${name}`);
  }
  
  // Reset to block
  setCursorShape(CursorShape.Block);
  console.log(`  Reset to Block`);
}

function demoProgressBar() {
  printSection("═══ Progress Bar Demo ═══");
  
  console.log("Animated progress bar:");
  
  // Simple progress bar
  for (let i = 0; i <= 100; i += 10) {
    printProgress(i, 100, 30);
    // Simple delay
    const start = Date.now();
    while (Date.now() - start < 100) {}
  }
  
  console.log("\n");
}

function demoSpinner() {
  printSection("═══ Spinner Demo ═══");
  
  console.log("Spinner animation (5 frames):");
  const frames = 5;
  for (let i = 0; i < frames; i++) {
    const spinner = getSpinnerFrame(i);
    process.stdout.write(`\r  ${spinner} Loading...`);
    const start = Date.now();
    while (Date.now() - start < 200) {}
  }
  console.log(`\n  ✓ Done!`);
}

function demoTerminalTitle() {
  printSection("═══ Terminal Title Demo ═══");
  
  const titles = [
    "stdio-napi Demo",
    "🤖 Working...",
    "📺 Terminal Title",
    "🔧 Build Complete!",
  ];
  
  for (const title of titles) {
    setTerminalTitle(title);
    console.log(`  Set title: "${title}"`);
  }
  
  // Reset title
  setTerminalTitle("Terminal");
}

async function demoInteractiveInput() {
  printSection("═══ Interactive Input Demo ═══");
  
  if (!isStdinTty()) {
    console.log("  Skipped (not running in interactive terminal)");
    return;
  }
  
  console.log("Testing interactive input with readLineInteractive:");
  console.log("  (Type something and press Enter)");
  
  try {
    const input = await readLineInteractive("  > ", []);
    console.log(`  You entered: "${input}"`);
  } catch (e) {
    printError(`  Input error: ${e}`);
  }
}

async function demoRawMode() {
  printSection("═══ Raw Mode Demo ═══");
  
  if (!isStdinTty()) {
    console.log("  Skipped (not running in interactive terminal)");
    return;
  }
  
  console.log("Enabling raw mode...");
  enableRawMode();
  
  console.log("  Raw mode enabled (input is not buffered)");
  console.log("  Press any key...");
  
  // Note: This would require key event handling which is complex
  // For demo purposes, we'll just disable raw mode
  disableRawMode();
  
  console.log("  Raw mode disabled");
}

async function runFullDemo() {
  console.clear();
  
  printHeader("╔════════════════════════════════════════╗");
  printHeader("║     Terminal/TTY Features Demo        ║");
  printHeader("╚════════════════════════════════════════╝");
  
  console.log(`\n${colors.cyan}This demo showcases the terminal features available in stdio-napi${colors.reset}`);
  
  // Run demos
  demoBasicInfo();
  demoCursorMovement();
  demoScreenClearing();
  demoAlternateScreen();
  demoCursorVisibility();
  demoCursorShapes();
  demoProgressBar();
  demoSpinner();
  demoTerminalTitle();
  
  await demoInteractiveInput();
  await demoRawMode();
  
  // Reset terminal state
  showCursor();
  setCursorShape(CursorShape.Block);
  leaveAlternateScreen();
  setTerminalTitle("Terminal");
  
  printHeader("\n═══ Demo Complete ═══");
  printSuccess("All terminal features demonstrated successfully!");
  
  // Return to original position
  moveCursor(0, 0);
}

// Run the demo
runFullDemo().catch((e) => {
  printError(`Demo failed: ${e}`);
  // Make sure to restore terminal state on error
  try {
    showCursor();
    disableRawMode();
    leaveAlternateScreen();
  } catch {}
});
