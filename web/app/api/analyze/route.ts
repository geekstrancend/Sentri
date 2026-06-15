import { NextRequest, NextResponse } from 'next/server'

export async function POST(request: NextRequest) {
  try {
    const { code } = await request.json()

    if (!code || typeof code !== 'string') {
      return NextResponse.json({ error: 'Code is required' }, { status: 400 })
    }

    const apiKey = process.env.ANTHROPIC_API_KEY
    if (!apiKey) {
      return NextResponse.json(
        { error: 'ANTHROPIC_API_KEY environment variable is not set' },
        { status: 500 },
      )
    }

    // Call Anthropic API
    const response = await fetch('https://api.anthropic.com/v1/messages', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': apiKey,
        'anthropic-version': '2023-06-01',
      },
      body: JSON.stringify({
        model: 'claude-haiku-4-5-20251001',
        max_tokens: 1024,
        messages: [
          {
            role: 'user',
            content: `You are a smart contract security expert. Analyze the following code for vulnerabilities and security issues.

Code to analyze:
\`\`\`
${code}
\`\`\`

Provide your response in JSON format with the following structure:
{
  "riskLevel": "low|medium|high|critical",
  "summary": "Brief overview of the code and main findings",
  "vulnerabilities": ["vulnerability 1", "vulnerability 2", ...],
  "recommendations": ["recommendation 1", "recommendation 2", ...]
}

Be concise but thorough. Focus on high-impact security issues.`,
          },
        ],
      }),
    })

    if (!response.ok) {
      const error = await response.text()
      console.error('Anthropic API error:', error)
      return NextResponse.json(
        { error: 'Failed to analyze code with Claude Haiku' },
        { status: 500 },
      )
    }

    const data = await response.json()

    // Extract the text response
    const textContent = data.content.find((block: any) => block.type === 'text')
    if (!textContent) {
      return NextResponse.json(
        { error: 'No text response from Claude Haiku' },
        { status: 500 },
      )
    }

    // Parse the JSON response
    try {
      const analysisText = textContent.text
      // Extract JSON from the response (it might be wrapped in markdown code blocks)
      const jsonMatch = analysisText.match(/\{[\s\S]*\}/)
      if (!jsonMatch) {
        throw new Error('No JSON found in response')
      }
      const analysis = JSON.parse(jsonMatch[0])

      return NextResponse.json(analysis)
    } catch (parseError) {
      console.error('Error parsing Claude response:', parseError)
      // Return a default analysis if parsing fails
      return NextResponse.json({
        riskLevel: 'medium',
        summary: 'Analysis complete. Please review the code for potential security issues.',
        vulnerabilities: ['Unable to parse detailed analysis'],
        recommendations: ['Enable verbose logging for detailed results'],
      })
    }
  } catch (error) {
    console.error('API error:', error)
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 },
    )
  }
}
