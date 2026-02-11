#!/bin/bash
cd /home/zelius/invar
echo "Current git status:"
git status --short | head -20

echo ""
echo "Pushing to GitHub..."
git push origin main --force -q && echo "✓ Push successful!" || echo "✗ Push failed"
