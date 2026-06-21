// Mobile Navigation Toggle
const mobileMenuBtn = document.querySelector('.mobile-menu-btn');
const navLinks = document.querySelector('.nav-links');

if (mobileMenuBtn) {
  mobileMenuBtn.addEventListener('click', () => {
    navLinks.classList.toggle('active');
  });
}

// Close mobile menu when clicking a link
document.querySelectorAll('.nav-links a').forEach(link => {
  link.addEventListener('click', () => {
    navLinks.classList.remove('active');
  });
});

// Playground Functionality
const runBtn = document.querySelector('.run-btn');
const codeEditor = document.querySelector('.code-editor');
const outputContent = document.querySelector('.output-content');

if (runBtn && codeEditor && outputContent) {
  runBtn.addEventListener('click', () => {
    const code = codeEditor.value;
    simulateRun(code);
  });
}

function simulateRun(code) {
  outputContent.innerHTML = '';
  
  // Simple simulation - parse print statements and basic operations
  const lines = code.split('\n');
  
  lines.forEach(line => {
    line = line.trim();
    
    // Skip empty lines and comments
    if (!line || line.startsWith('//') || line.startsWith('#')) {
      return;
    }
    
    // Handle print statements
    const printMatch = line.match(/print\((.+)\)/);
    if (printMatch) {
      let content = printMatch[1];
      
      // Remove quotes for strings
      if ((content.startsWith('"') && content.endsWith('"')) || 
          (content.startsWith("'") && content.endsWith("'"))) {
        content = content.slice(1, -1);
      }
      
      // Handle variable references (simple lookup)
      if (variables[content]) {
        content = variables[content];
      }
      
      outputContent.innerHTML += `<div>${content}</div>`;
      return;
    }
    
    // Handle variable assignments
    const varMatch = line.match(/let\s+(\w+)\s*=\s*(.+)/);
    if (varMatch) {
      variables[varMatch[1]] = varMatch[2];
      return;
    }
    
    // Handle function calls (simple simulation)
    if (line.includes('Math.random()')) {
      outputContent.innerHTML += `<div>${Math.random().toFixed(2)}</div>`;
    }
  });
  
  if (outputContent.innerHTML === '') {
    outputContent.innerHTML = '<div class="success">// Program executed successfully</div>';
  }
}

const variables = {};

// Search functionality for packages page
const searchInput = document.querySelector('.search-bar input');
if (searchInput) {
  searchInput.addEventListener('input', (e) => {
    const searchTerm = e.target.value.toLowerCase();
    const packageCards = document.querySelectorAll('.package-card');
    
    packageCards.forEach(card => {
      const name = card.querySelector('.package-name').textContent.toLowerCase();
      const desc = card.querySelector('p').textContent.toLowerCase();
      
      if (name.includes(searchTerm) || desc.includes(searchTerm)) {
        card.style.display = 'block';
      } else {
        card.style.display = 'none';
      }
    });
  });
}

// Category filter for packages
const categoryBtns = document.querySelectorAll('.category-btn');
categoryBtns.forEach(btn => {
  btn.addEventListener('click', () => {
    categoryBtns.forEach(b => b.classList.remove('active'));
    btn.classList.add('active');
    
    const category = btn.dataset.category;
    const packageCards = document.querySelectorAll('.package-card');
    
    packageCards.forEach(card => {
      if (category === 'all' || card.dataset.category === category) {
        card.style.display = 'block';
      } else {
        card.style.display = 'none';
      }
    });
  });
});

// Smooth scroll for anchor links
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
  anchor.addEventListener('click', function(e) {
    e.preventDefault();
    const target = document.querySelector(this.getAttribute('href'));
    if (target) {
      target.scrollIntoView({
        behavior: 'smooth',
        block: 'start'
      });
    }
  });
});

// Active sidebar link highlighting for docs
const sidebarLinks = document.querySelectorAll('.sidebar-links a');
if (sidebarLinks.length > 0) {
  sidebarLinks.forEach(link => {
    link.addEventListener('click', function() {
      sidebarLinks.forEach(l => l.classList.remove('active'));
      this.classList.add('active');
    });
  });
}

// Navbar background change on scroll
window.addEventListener('scroll', () => {
  const navbar = document.querySelector('.navbar');
  if (navbar) {
    if (window.scrollY > 50) {
      navbar.style.background = 'rgba(10, 10, 10, 0.95)';
    } else {
      navbar.style.background = 'rgba(10, 10, 10, 0.8)';
    }
  }
});

// Copy code block functionality
document.querySelectorAll('.code-content').forEach(block => {
  block.style.cursor = 'pointer';
  block.title = 'Click to copy';
  
  block.addEventListener('click', () => {
    const text = block.textContent;
    navigator.clipboard.writeText(text).then(() => {
      const originalText = block.innerHTML;
      block.innerHTML = '<span style="color: #34d399">Copied!</span>';
      setTimeout(() => {
        block.innerHTML = originalText;
      }, 2000);
    });
  });
});
