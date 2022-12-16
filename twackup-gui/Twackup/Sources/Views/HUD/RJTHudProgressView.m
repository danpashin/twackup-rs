//
//  RJTHudProgressView.m
//  RJTranslate
//
//  Created by Даниил on 28/10/2018.
//  Copyright © 2018 Даниил. All rights reserved.
//

#import "RJTHudProgressView.h"

@interface RJTHudProgressView ()
@property (nonatomic, readonly, strong) CAShapeLayer *layer;
@property (nonatomic, readonly, assign) BOOL basicAnimation;
@end

@implementation RJTHudProgressView
@dynamic layer;

+ (Class)layerClass {
    return [CAShapeLayer class];
}

+ (instancetype)defaultProgressView {
    return [[RJTHudProgressView alloc] initWithFrame:CGRectMake(0.0, 0.0, 66.0, 66.0)];
}

- (instancetype)initWithFrame:(CGRect)frame {
    CGFloat size = (CGRectGetWidth(frame) + CGRectGetHeight(frame)) / 2.0;
    frame.size = CGSizeMake(size, size);
    
    self = [super initWithFrame:frame];
    if (self) {
        CGFloat center = size / 2.0;
        
        CGMutablePathRef mutablePath = CGPathCreateMutable();
        CGPathAddArc(mutablePath, NULL, center, center, center - 8.0, 0.0, (CGFloat)(2.0 * M_PI), NO);

        CAShapeLayer *layer = self.layer;
        layer.path = mutablePath;
        layer.fillColor = [UIColor clearColor].CGColor;
        layer.lineCap = kCALineCapRound;
        layer.lineWidth = 3.0;
        layer.strokeEnd = 0.0;
        
        CGPathRelease(mutablePath);
        
        self.progress = 0.0;
        
        [[NSNotificationCenter defaultCenter] addObserver:self selector:@selector(applicationDidBecomeActive) name:UIApplicationDidBecomeActiveNotification object:nil];
        [[NSNotificationCenter defaultCenter] addObserver:self selector:@selector(applicationDidEnterBackground) name:UIApplicationDidEnterBackgroundNotification object:nil];
    }
    return self;
}

- (void)tintColorDidChange {
    [super tintColorDidChange];
    
    self.layer.strokeColor = self.tintColor.CGColor;
}

- (void)runBasicAnimation {
    [self.layer removeAllAnimations];

    _animating = YES;
    _basicAnimation = YES;

    CABasicAnimation *rotateAnimation = [CABasicAnimation animationWithKeyPath:@"transform.rotation.z"];
    rotateAnimation.fromValue = @0.0;
    rotateAnimation.toValue = @(M_PI * 2.0);
    rotateAnimation.duration = 1.75;
    rotateAnimation.repeatCount = INFINITY;
    [self.layer addAnimation:rotateAnimation forKey:@"rotate"];
}

- (void)runSpinnerAnimation {
    [self.layer removeAllAnimations];

    [self runBasicAnimation];
    _basicAnimation = NO;

    const CFTimeInterval strokeDuration = 2.25;

    CABasicAnimation *startAnim = [CABasicAnimation animationWithKeyPath:@"strokeStart"];
    startAnim.beginTime = strokeDuration * 0.15;
    startAnim.fromValue = @0.0;
    startAnim.toValue = @0.85;
    startAnim.duration = strokeDuration;
    startAnim.timingFunction = [CAMediaTimingFunction functionWithControlPoints:0.2f :0.88f :0.09f :0.99f];
    startAnim.fillMode = kCAFillModeBackwards;

    CABasicAnimation *endAnim = [CABasicAnimation animationWithKeyPath:@"strokeEnd"];
    endAnim.beginTime = 0.0;
    endAnim.fromValue = @0.0;
    endAnim.toValue = @1.075;
    endAnim.duration = strokeDuration;
    endAnim.timingFunction = [CAMediaTimingFunction functionWithControlPoints:0.4f :0.88f :0.09f :0.99f];
    endAnim.fillMode = kCAFillModeForwards;

    CAAnimationGroup *strokeAnim = [CAAnimationGroup animation];
    strokeAnim.animations = @[endAnim, startAnim];
    strokeAnim.duration = strokeDuration;
    strokeAnim.repeatCount = INFINITY;

    [self.layer addAnimation:strokeAnim forKey:@"strokeFill"];
}

- (void)stopAnimating {
    _animating = NO;
    [self.layer removeAllAnimations];
}

- (void)setProgress:(CGFloat)progress {
    [self setProgress:progress animated:NO];
}

- (void)setProgress:(CGFloat)progress animated:(BOOL)animated {
    _progress = progress;
    
    if (animated) {
        CABasicAnimation *strokeEndAnimation = [CABasicAnimation animationWithKeyPath:@"strokeEnd"];
        strokeEndAnimation.fromValue = @(self.layer.strokeEnd);
        strokeEndAnimation.toValue = @(progress);
        strokeEndAnimation.duration = 1.0;
        [self.layer addAnimation:strokeEndAnimation forKey:@"strokeEnd"];
    }
    
    self.layer.strokeEnd = progress;
}

- (void)didMoveToSuperview {
    [super didMoveToSuperview];

    _cachedHeight = CGRectGetHeight(self.frame);
    _heightConstraint = [self.heightAnchor constraintEqualToConstant:self.cachedHeight];

    [NSLayoutConstraint activateConstraints:@[
        _heightConstraint, [self.widthAnchor constraintEqualToConstant:CGRectGetWidth(self.frame)]
    ]];
}

- (void)applicationDidBecomeActive {
    if (self.animating && self.basicAnimation) {
        [self runBasicAnimation];
    } else if (self.animating) {
        [self runSpinnerAnimation];
    }
}

- (void)applicationDidEnterBackground {
    [self.layer removeAllAnimations];
}

- (void)dealloc {
    [[NSNotificationCenter defaultCenter] removeObserver:self];
}

@end
