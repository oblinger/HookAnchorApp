
Roll Up

```dataviewjs
// Simple Date View Script
const files = dv.pages();
const entries = [];

for (const file of files) {
    if (!file?.file?.path?.endsWith('.md')) continue;
    
    // Check if filename starts with YYYY-MM-DD
    const fileName = file.file.name.replace(/\.md$/, '');
    const fileNameDateMatch = fileName.match(/^(\d{4}-\d{2}-\d{2})/);
    
    if (fileNameDateMatch) {
        // Add the file itself as an entry
        entries.push({
            date: fileNameDateMatch[1],
            text: fileName, // Use filename as display text
            link: fileName
        });
    }
    
    try {
        const content = await dv.io.load(file.file.path);
        if (!content) continue;
        
        const lines = content.split('\n');
        lines.forEach((line, index) => {
            if (line.startsWith('### ')) {
                const headerText = line.substring(4);
                const dateMatch = headerText.match(/^(\d{4}-\d{2}-\d{2})/);
                
                if (dateMatch) {
                    // Get first line after header
                    let firstLine = "No content";
                    for (let i = index + 1; i < lines.length; i++) {
                        const nextLine = lines[i].trim();
                        if (nextLine && !nextLine.startsWith('#')) {
                            firstLine = nextLine;
                            break;
                        }
                        if (nextLine.startsWith('#')) break;
                    }
                    
                    // Create proper Obsidian link anchor - use just filename without path
                    const fileNameNoExt = file.file.name.replace(/\.md$/, '');
                    // Clean header: remove URLs, brackets, pipes, and other problematic characters
                    const cleanHeader = headerText
                        .replace(/\(https?:\/\/[^\)]+\)/g, '') // Remove URLs in parentheses
                        .replace(/https?:\/\/\S+/g, '') // Remove standalone URLs
                        .replace(/[#\[\]|()]/g, '') // Remove problematic characters
                        .trim();
                    const linkTarget = fileNameNoExt + '#' + cleanHeader;
                    
                    entries.push({
                        date: dateMatch[1],
                        text: cleanHeader, // Use the cleaned header as display text
                        link: linkTarget
                    });
                }
            }
        });
    } catch (error) {
        // Skip problematic files
    }
}

// Sort by date (newest first)
entries.sort((a, b) => b.date.localeCompare(a.date));



// Display results
if (entries.length === 0) {
    dv.paragraph("No date entries found");
} else {
    dv.header(2, `Date Entries (${entries.length})`);
    
    dv.table(["Timeline"], 
        entries.map(e => {
            const fileName = e.link.includes('#') ? e.link.split('#')[0] : e.link;
            const displayFileName = fileName.length > 20 ? fileName.substring(0, 20) : fileName;
            // Add non-breaking spaces before filename in parentheses
            const nbspSpaces = '\u00A0'.repeat(3); // 3 non-breaking spaces
            return [`[[${e.link}|${e.text}]]${nbspSpaces}([[${fileName}|${displayFileName}]])`];
        })
    );
}
```