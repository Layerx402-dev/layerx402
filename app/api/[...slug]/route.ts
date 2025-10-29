import { NextRequest, NextResponse } from 'next/server'

// Catch-all API route that returns 401 Unauthorized
export async function GET(request: NextRequest) {
  return NextResponse.json(
    { 
      error: 'Unauthorized',
      message: 'Layerx402 API access requires valid authentication',
      status: 401 
    },
    { status: 401 }
  )
}

export async function POST(request: NextRequest) {
  return NextResponse.json(
    { 
      error: 'Unauthorized',
      message: 'Layerx402 API access requires valid authentication',
      status: 401 
    },
    { status: 401 }
  )
}

export async function PUT(request: NextRequest) {
  return NextResponse.json(
    { 
      error: 'Unauthorized',
      message: 'Layerx402 API access requires valid authentication',
      status: 401 
    },
    { status: 401 }
  )
}

export async function DELETE(request: NextRequest) {
  return NextResponse.json(
    { 
      error: 'Unauthorized',
      message: 'Layerx402 API access requires valid authentication',
      status: 401 
    },
    { status: 401 }
  )
}

export async function PATCH(request: NextRequest) {
  return NextResponse.json(
    { 
      error: 'Unauthorized',
      message: 'Layerx402 API access requires valid authentication',
      status: 401 
    },
    { status: 401 }
  )
}

export async function OPTIONS(request: NextRequest) {
  return NextResponse.json(
    { 
      error: 'Unauthorized',
      message: 'Layerx402 API access requires valid authentication',
      status: 401 
    },
    { status: 401 }
  )
}