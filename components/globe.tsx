'use client'

import { useEffect, useRef } from 'react'
import * as THREE from 'three'

export function Globe() {
  const containerRef = useRef<HTMLDivElement>(null)
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null)
  const sceneRef = useRef<THREE.Scene | null>(null)
  const cameraRef = useRef<THREE.PerspectiveCamera | null>(null)
  const globeRef = useRef<THREE.Mesh | null>(null)

  useEffect(() => {
    if (!containerRef.current) return

    // Scene setup
    const scene = new THREE.Scene()
    sceneRef.current = scene

    // Camera setup
    const camera = new THREE.PerspectiveCamera(
      45,
      window.innerWidth / window.innerHeight,
      0.1,
      1000
    )
    camera.position.z = 3
    cameraRef.current = camera

    // Renderer setup
    const renderer = new THREE.WebGLRenderer({
      alpha: true,
      antialias: true,
    })
    renderer.setSize(window.innerWidth, window.innerHeight)
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
    containerRef.current.appendChild(renderer.domElement)
    rendererRef.current = renderer

    // Create globe
    const globeGeometry = new THREE.SphereGeometry(1, 64, 64)

    // Create globe material - deep ocean color
    const globeMaterial = new THREE.MeshBasicMaterial({
      color: 0x0D2123, // Deep ocean
      wireframe: false,
      transparent: true,
      opacity: 0.3,
    })

    const globe = new THREE.Mesh(globeGeometry, globeMaterial)
    scene.add(globe)
    globeRef.current = globe

    // Add wireframe overlay - Neon teal color
    const wireframeGeometry = new THREE.SphereGeometry(1.002, 32, 32)
    const wireframeMaterial = new THREE.MeshBasicMaterial({
      color: 0x00FFD1, // Neon teal
      wireframe: true,
      transparent: true,
      opacity: 0.2,
    })
    const wireframe = new THREE.Mesh(wireframeGeometry, wireframeMaterial)
    scene.add(wireframe)

    // Add secondary wireframe with electric cyan for depth
    const wireframe2Geometry = new THREE.SphereGeometry(1.001, 24, 24)
    const wireframe2Material = new THREE.MeshBasicMaterial({
      color: 0x00E5CC, // Electric cyan
      wireframe: true,
      transparent: true,
      opacity: 0.1,
    })
    const wireframe2 = new THREE.Mesh(wireframe2Geometry, wireframe2Material)
    scene.add(wireframe2)

    // Add points (cities/nodes) on globe
    const pointsGeometry = new THREE.BufferGeometry()
    const pointsCount = 100
    const positions = new Float32Array(pointsCount * 3)

    for (let i = 0; i < pointsCount; i++) {
      const theta = Math.random() * Math.PI * 2
      const phi = Math.acos(2 * Math.random() - 1)
      const radius = 1.005

      positions[i * 3] = radius * Math.sin(phi) * Math.cos(theta)
      positions[i * 3 + 1] = radius * Math.sin(phi) * Math.sin(theta)
      positions[i * 3 + 2] = radius * Math.cos(phi)
    }

    pointsGeometry.setAttribute('position', new THREE.BufferAttribute(positions, 3))

    const pointsMaterial = new THREE.PointsMaterial({
      color: 0x00FFD1, // Neon teal
      size: 0.01,
      transparent: true,
      opacity: 0.8,
    })

    const points = new THREE.Points(pointsGeometry, pointsMaterial)
    scene.add(points)

    // Animation loop
    let animationId: number
    let time = 0

    const animate = () => {
      animationId = requestAnimationFrame(animate)
      time += 0.01

      // Rotate globe slowly
      if (globeRef.current) {
        globeRef.current.rotation.y += 0.001
        wireframe.rotation.y += 0.001
        wireframe2.rotation.y -= 0.0008 // Counter-rotate for visual interest
        points.rotation.y += 0.001
      }

      // Pulsate points with sine wave
      const pulseSize = 0.01 + Math.sin(time) * 0.003
      const pulseOpacity = 0.6 + Math.sin(time) * 0.2
      pointsMaterial.size = pulseSize
      pointsMaterial.opacity = pulseOpacity

      renderer.render(scene, camera)
    }

    animate()

    // Handle resize
    const handleResize = () => {
      if (!cameraRef.current || !rendererRef.current) return

      cameraRef.current.aspect = window.innerWidth / window.innerHeight
      cameraRef.current.updateProjectionMatrix()
      rendererRef.current.setSize(window.innerWidth, window.innerHeight)
    }

    window.addEventListener('resize', handleResize)

    // Cleanup
    return () => {
      window.removeEventListener('resize', handleResize)
      cancelAnimationFrame(animationId)

      if (rendererRef.current && containerRef.current) {
        containerRef.current.removeChild(rendererRef.current.domElement)
        rendererRef.current.dispose()
      }

      // Dispose geometries and materials
      globeGeometry.dispose()
      globeMaterial.dispose()
      wireframeGeometry.dispose()
      wireframeMaterial.dispose()
      wireframe2Geometry.dispose()
      wireframe2Material.dispose()
      pointsGeometry.dispose()
      pointsMaterial.dispose()
    }
  }, [])

  return (
    <div
      ref={containerRef}
      className="fixed inset-0 z-0"
      style={{
        background: 'radial-gradient(circle at center, #0D2123 0%, #0A0E14 100%)',
      }}
    />
  )
}
