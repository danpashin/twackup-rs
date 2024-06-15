//
//  HudCircularProgressView.swift
//  iAppsDRM
//
//  Created by Daniil on 10.01.2024.
//

import UIKit

final class HudCircularProgressView: UIView {
    enum AnimationStyle {
        case arc, arcRotate
    }

    override final class var layerClass: AnyClass {
        CAShapeLayer.self
    }

    var animationStyle: AnimationStyle = .arc {
        didSet {
            runAnimation()
        }
    }

    private(set) var progress: CGFloat = 0.75

    private(set) var isAnimating = false

    private let arcSize: CGSize

    override var layer: CAShapeLayer {
        super.layer as! CAShapeLayer
    }

    override var intrinsicContentSize: CGSize {
        isAnimating ? arcSize : .zero
    }

    override init(frame: CGRect) {
        let diameter = min(frame.height, frame.width)
        let center = CGPoint(x: frame.width / 2, y: frame.height / 2)
        arcSize = CGSize(width: diameter, height: diameter)

        super.init(frame: frame)

        layer.fillColor = UIColor.clear.cgColor
        layer.lineCap = .round
        layer.lineWidth = 4.0
        layer.strokeEnd = 0.5

        let arcPath = CGMutablePath()
        arcPath.addArc(
            center: center,
            radius: (diameter - layer.lineWidth) / 2,
            startAngle: 0.0,
            endAngle: 2.0 * .pi,
            clockwise: false
        )
        layer.path = arcPath

        let notificationCenter = NotificationCenter.default
        notificationCenter.addObserver(
            self,
            selector: #selector(applicationDidBecomeActive),
            name: UIApplication.didBecomeActiveNotification,
            object: nil
        )
        notificationCenter.addObserver(
            self,
            selector: #selector(applicationDidEnterBackground),
            name: UIApplication.didEnterBackgroundNotification,
            object: nil
        )
    }

    @available(*, unavailable)
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func tintColorDidChange() {
        super.tintColorDidChange()

        layer.strokeColor = tintColor.cgColor
    }

    private func runArcAnimation() {
        let animation = CABasicAnimation(keyPath: "transform.rotation.z")
        animation.fromValue = 0.0
        animation.toValue = Double.pi * 2.0
        animation.duration = 1.75
        animation.isCumulative = true
        animation.repeatCount = .greatestFiniteMagnitude

        layer.add(animation, forKey: "rotate")
    }

    private func runArcDotAnimation() {
        runArcAnimation()

        let strokeDuration: CFTimeInterval = 2.25

        let startAnimation = CABasicAnimation(keyPath: "strokeStart")
        startAnimation.beginTime = strokeDuration * 0.15
        startAnimation.fromValue = 0.0
        startAnimation.toValue = 0.85
        startAnimation.duration = strokeDuration
        startAnimation.timingFunction = CAMediaTimingFunction(controlPoints: 0.2, 0.88, 0.09, 0.99)
        startAnimation.fillMode = .backwards

        let endAnimation = CABasicAnimation(keyPath: "strokeEnd")
        endAnimation.fromValue = 0.0
        endAnimation.toValue = 1.075
        endAnimation.duration = strokeDuration
        endAnimation.timingFunction = CAMediaTimingFunction(controlPoints: 0.4, 0.88, 0.09, 0.99)
        endAnimation.fillMode = .forwards

        let strokeAnimation = CAAnimationGroup()
        strokeAnimation.animations = [endAnimation, startAnimation]
        strokeAnimation.duration = strokeDuration
        strokeAnimation.repeatCount = .greatestFiniteMagnitude

        layer.add(strokeAnimation, forKey: "strokeFill")
    }

    // MARK: - Public methods

    func runAnimation() {
        stopAnimation()
        isAnimating = true

        setProgress(progress, animated: false)

        switch animationStyle {
        case .arc:
            runArcAnimation()

        case .arcRotate:
            runArcDotAnimation()
        }
    }

    func stopAnimation() {
        isAnimating = false

        layer.removeAllAnimations()
        layer.strokeEnd = 0.0
    }

    func setProgress(_ progress: CGFloat, animated: Bool) {
        self.progress = progress

        if animated {
            let animation = CABasicAnimation(keyPath: "strokeEnd")
            animation.fromValue = layer.strokeEnd
            animation.toValue = progress
            animation.duration = 1.0

            layer.add(animation, forKey: "strokeEnd")
        }

        layer.strokeEnd = progress
    }

    // MARK: - Private methods

    @objc
    private func applicationDidBecomeActive() {
        if isAnimating {
            runAnimation()
        }
    }

    @objc
    private func applicationDidEnterBackground() {
        layer.removeAllAnimations()
    }

    deinit {
        NotificationCenter.default.removeObserver(self)
    }
}
