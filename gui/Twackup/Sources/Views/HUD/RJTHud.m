//
//  RJTHud.m
//  RJTranslate
//
//  Created by Даниил on 28/10/2018.
//  Copyright © 2018 Даниил. All rights reserved.
//

#import "RJTHud.h"
#import "RJTHudProgressView.h"

RJTHUD_OBJC_DIRECT_MEMBERS
@interface RJTHud ()
@property (strong, nonatomic RJTHUD_DIRECT) UIView *contentView;
@property (strong, nonatomic RJTHUD_DIRECT) UIVisualEffectView *blurView;

@property (strong, nonatomic RJTHUD_DIRECT) RJTHudProgressView *progressView;
@property (strong, nonatomic RJTHUD_DIRECT) UILabel *textLabel;
@property (strong, nonatomic RJTHUD_DIRECT) UILabel *detailedTextLabel;

@property (strong, nonatomic RJTHUD_DIRECT) NSLayoutConstraint *widthConstraint;
@property (strong, nonatomic RJTHUD_DIRECT) NSLayoutConstraint *heightConstraint;

@property (strong, nonatomic RJTHUD_DIRECT) NSLayoutConstraint *textLabelHeightConstraint;
@property (strong, nonatomic RJTHUD_DIRECT) NSLayoutConstraint *detailedTextLabelHeightConstraint;

@property (assign, nonatomic, readonly RJTHUD_DIRECT) CGFloat labelsDefaultHeight;

- (void)runSpinner RJTHUD_OBJC_DIRECT;

@end

RJTHUD_OBJC_DIRECT_MEMBERS
@implementation RJTHud

static CGFloat const RJTHudContentSize = 140.0;

+ (nullable instancetype)show {
    for (UIScene *scene in UIApplication.sharedApplication.connectedScenes) {
        if (scene.activationState != UISceneActivationStateForegroundActive) {
            continue;
        }

        if (![scene isKindOfClass:[UIWindowScene class]]) {
            continue;
        }

        for (UIWindow *window in ((UIWindowScene *)scene).windows) {
            if (window.keyWindow) {
                RJTHud *hud = [RJTHud new];
                [hud showAnimated:YES onView:window];
                hud.progress = 0.75;

                return hud;
            }
        }
    }

    return nil;
}

- (instancetype)init {
    self = [super initWithFrame:CGRectZero];
    if (self) {
        _labelsDefaultHeight = 32.0;
        _blurStyle = UIBlurEffectStyleSystemThickMaterial;
        self.translatesAutoresizingMaskIntoConstraints = NO;

        [self createViewHierarchy];
    }
    return self;
}

#pragma mark - Views

- (UIView *)contentView {
    if (!_contentView) {
        _contentView = [UIView new];
        _contentView.backgroundColor = [UIColor colorWithWhite:1.0 alpha:0.3];
        _contentView.layer.cornerRadius = 24.0;
        _contentView.layer.cornerCurve = kCACornerCurveContinuous;

        _contentView.layer.shadowOpacity = 0.2f;
        _contentView.layer.shadowColor = UIColor.blackColor.CGColor;
        _contentView.layer.shadowRadius = 10.0;
        _contentView.layer.shadowOffset = CGSizeMake(0.0, 10.0);
    }

    return _contentView;
}

- (UIVisualEffectView *)blurView {
    if (!_blurView) {
        _blurView = [[UIVisualEffectView alloc] initWithEffect:[UIBlurEffect effectWithStyle:self.blurStyle]];
        _blurView.layer.masksToBounds = YES;
        _blurView.layer.cornerRadius = self.contentView.layer.cornerRadius;
        _blurView.layer.cornerCurve = kCACornerCurveContinuous;
    }

    return _blurView;
}

- (RJTHudProgressView *)progressView {
    if (!_progressView) {
        _progressView = RJTHudProgressView.defaultProgressView;
        _progressView.tintColor = UIColor.systemGrayColor;
    }

    return _progressView;
}

- (UILabel *)textLabel {
    if (!_textLabel) {
        _textLabel = [[UILabel alloc] init];
        _textLabel.numberOfLines = 1;
        _textLabel.textAlignment = NSTextAlignmentCenter;
        _textLabel.textColor = UIColor.labelColor;
        _textLabel.font = [UIFont boldSystemFontOfSize:[UIFont buttonFontSize]];
    }

    return _textLabel;
}

- (UILabel *)detailedTextLabel {
    if (!_detailedTextLabel) {
        _detailedTextLabel = [[UILabel alloc] init];
        _detailedTextLabel.numberOfLines = 0;
        _detailedTextLabel.textAlignment = NSTextAlignmentCenter;
        _detailedTextLabel.textColor = [UIColor secondaryLabelColor];
        _detailedTextLabel.font = [UIFont boldSystemFontOfSize:[UIFont systemFontSize]];
    }

    return _detailedTextLabel;
}

- (UIStackView *)stackView {
    UIStackView *stackView = [[UIStackView alloc] initWithArrangedSubviews:@[
        self.progressView, self.textLabel, self.detailedTextLabel
    ]];
    stackView.axis = UILayoutConstraintAxisVertical;
    stackView.distribution = UIStackViewDistributionFill;
    stackView.alignment = UIStackViewAlignmentCenter;

    return stackView;
}

#pragma mark -
#pragma mark Public
#pragma mark -

- (void)showAnimated:(BOOL)animated onView:(UIView *)view {
    [self performOnMainThread:^{
        if (animated) {
            self.alpha = 0.0;
            [view addSubview:self];

            [self animateWithDuration:0.25 animations:^{
                self.alpha = 1.0;
            } completion:nil];
        } else {
            [view addSubview:self];
        }

        [self runSpinner];
    }];
}

- (void)hideAnimated:(BOOL)animated {
    [self performOnMainThread:^{
        if (animated) {
            [self animateWithDuration:0.5 animations:^{
                self.alpha = 0.0;
            } completion:^(BOOL finished) {
                [self.progressView stopAnimating];
                [self removeFromSuperview];
            }];
        } else {
            [self.progressView stopAnimating];
            [self removeFromSuperview];
        }
    }];
}

- (void)hideAfterDelay:(CGFloat)delay {
    dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(delay * NSEC_PER_SEC)), dispatch_get_main_queue(), ^{
        [self hideAnimated:YES];
    });
}

- (void)setProgress:(CGFloat)progress {
    [self setProgress:progress animated:NO];
}

- (void)setProgress:(CGFloat)progress animated:(BOOL)animated {
    _progress = progress;

    [self performOnMainThread:^{
        [self.progressView setProgress:progress animated:animated];
    }];
}


#pragma mark - Private


- (void)createViewHierarchy {
    UIView *contentView = self.contentView;
    [self addSubview:contentView];

    UIVisualEffectView *blurView = self.blurView;
    [contentView addSubview:blurView];

    UIStackView *stackView = self.stackView;
    [blurView.contentView addSubview:stackView];


    stackView.translatesAutoresizingMaskIntoConstraints = NO;
    blurView.translatesAutoresizingMaskIntoConstraints = NO;
    contentView.translatesAutoresizingMaskIntoConstraints = NO;

    self.widthConstraint = [contentView.widthAnchor constraintEqualToConstant:RJTHudContentSize];
    self.heightConstraint = [contentView.heightAnchor constraintEqualToConstant:RJTHudContentSize];
    self.textLabelHeightConstraint = [self.textLabel.heightAnchor constraintEqualToConstant:0.0];
    self.detailedTextLabelHeightConstraint = [self.detailedTextLabel.heightAnchor constraintEqualToConstant:0.0];

    [NSLayoutConstraint activateConstraints:@[
        [contentView.centerXAnchor constraintEqualToAnchor:self.centerXAnchor],
        [contentView.centerYAnchor constraintEqualToAnchor:self.centerYAnchor],

        self.widthConstraint, self.heightConstraint,
        self.textLabelHeightConstraint, self.detailedTextLabelHeightConstraint,

        [stackView.centerXAnchor constraintEqualToAnchor:blurView.contentView.centerXAnchor],
        [stackView.centerYAnchor constraintEqualToAnchor:blurView.contentView.centerYAnchor],

        [blurView.topAnchor constraintEqualToAnchor:contentView.topAnchor],
        [blurView.bottomAnchor constraintEqualToAnchor:contentView.bottomAnchor],
        [blurView.leadingAnchor constraintEqualToAnchor:contentView.leadingAnchor],
        [blurView.trailingAnchor constraintEqualToAnchor:contentView.trailingAnchor],
    ]];
}

- (void)didMoveToSuperview {
    [super didMoveToSuperview];

    UIView *superview = self.superview;
    if (!superview) {
        return;
    }

    [NSLayoutConstraint activateConstraints:@[
        [self.topAnchor constraintEqualToAnchor:superview.topAnchor],
        [self.bottomAnchor constraintEqualToAnchor:superview.bottomAnchor],
        [self.leadingAnchor constraintEqualToAnchor:superview.leadingAnchor],
        [self.trailingAnchor constraintEqualToAnchor:superview.trailingAnchor],
    ]];
}

- (void)setText:(NSString *)text {
    if (!text.length) {
        text = nil;
    }

    _text = text;
    [self setText:text forLabel:self.textLabel heightConstraint:self.textLabelHeightConstraint];
}

- (void)setDetailedText:(NSString *)detailedText {
    if (!detailedText.length) {
        detailedText = nil;
    }

    _detailedText = detailedText;
    [self setText:detailedText forLabel:self.detailedTextLabel heightConstraint:self.detailedTextLabelHeightConstraint];
}

- (void)setBlurStyle:(UIBlurEffectStyle)blurStyle {
    _blurStyle = blurStyle;

    [self performOnMainThread:^{
        self.blurView.effect = [UIBlurEffect effectWithStyle:blurStyle];
    }];
}

- (void)setStyle:(RJTHudStyle)style {
    _style = style;

    [self performOnMainThread:^{
        if (self.style == RJTHudStyleProgress || self.style == RJTHudStyleSpinner) {
            [self runSpinner];

            self.progressView.heightConstraint.constant = self.progressView.cachedHeight;
            [self updateHudSizeWithCompletion:^{
                [self animateWithDuration:0.2 animations:^{
                    self.progressView.alpha = 1.0;
                } completion:nil];
            }];
        } else {
            [self animateWithDuration:0.2 animations:^{
                self.progressView.alpha = 0.0;
            } completion:^(BOOL finished) {
                [self.progressView stopAnimating];
                self.progressView.heightConstraint.constant = 0.0;
                [self updateHudSizeWithCompletion:nil];
            }];
        }
    }];
}



#pragma mark -

- (void)setText:(NSString *)text forLabel:(UILabel *)label heightConstraint:(NSLayoutConstraint *)heightConstraint {
    [self performOnMainThread:^{
        CATransition *textTransition = [CATransition animation];
        textTransition.timingFunction = [CAMediaTimingFunction functionWithName:kCAMediaTimingFunctionEaseInEaseOut];
        textTransition.type = kCATransitionFade;
        textTransition.duration = 0.5;
        [label.layer addAnimation:textTransition forKey:@"textChangeAnimation"];
        label.text = text;

        dispatch_after(dispatch_time(DISPATCH_TIME_NOW, (int64_t)(0.2 * NSEC_PER_SEC)), dispatch_get_main_queue(), ^{
            [self updateHudSizeWithCompletion:nil];
        });
    }];
}

- (void)updateHudSizeWithCompletion:(void(^)(void))completion {
    CGSize textSize = [self sizeForTextInLabel:self.textLabel];
    CGSize detailTextSize = [self sizeForTextInLabel:self.detailedTextLabel];

    CGFloat width = 32.0;
    if (textSize.width > RJTHudContentSize || detailTextSize.width > RJTHudContentSize) {
        width += MAX(textSize.width, detailTextSize.width);
    } else {
        width = RJTHudContentSize;
    }
    self.widthConstraint.constant = width;


    CGFloat progressViewHeight = self.progressView.heightConstraint.constant;
    CGFloat minLabelHeight = self.labelsDefaultHeight;

    CGFloat height = progressViewHeight + MAX(textSize.height, minLabelHeight) + MAX(detailTextSize.height, minLabelHeight);
    if (height > RJTHudContentSize)
        height += (CGFloat)32.0;

    self.heightConstraint.constant = height;
    self.textLabelHeightConstraint.constant = textSize.height;
    self.detailedTextLabelHeightConstraint.constant = detailTextSize.height;

    const CGFloat cornerRadius = self.contentView.layer.cornerRadius;
    CGPathRef shadowPath = CGPathCreateWithRoundedRect(CGRectMake(0.0, 0.0, width, height), cornerRadius, cornerRadius, NULL);
    self.contentView.layer.shadowPath = shadowPath;
    CGPathRelease(shadowPath);

    [self animateWithDuration:0.3 animations:^{
        [self.contentView layoutIfNeeded];
    } completion:^(BOOL finished) {
        if (completion)
            completion();
    }];
}

- (void)animateWithDuration:(NSTimeInterval)duration animations:(void(^)(void))animations
                 completion:(void (^ __nullable)(BOOL finished))completion {
    NSAssert([NSThread isMainThread], @"%s must be called only from the main thread!", __FUNCTION__);
    [UIView animateWithDuration:duration delay:0.0
                        options:UIViewAnimationOptionAllowAnimatedContent | UIViewAnimationOptionAllowUserInteraction
                     animations:animations completion:completion];
}

- (void)performOnMainThread:(void(^)(void))block {
    [NSThread isMainThread] ? block() : dispatch_sync(dispatch_get_main_queue(), block);
}

- (CGSize)sizeForTextInLabel:(UILabel *)label {
    NSString *text = label.text;
    NSUInteger linesCount = [text componentsSeparatedByCharactersInSet:[NSCharacterSet newlineCharacterSet]].count + 1;

    CGSize boundingSize = CGSizeMake(CGRectGetWidth([UIScreen mainScreen].bounds) - 16.0, self.labelsDefaultHeight);
    CGSize textSize = [text boundingRectWithSize:boundingSize
                                         options:0 attributes:@{NSFontAttributeName:label.font} context:nil].size;
    textSize.height *= (CGFloat)linesCount;

    return textSize;
}

- (void)runSpinner {
    switch (self.style) {
        case RJTHudStyleSpinner:
            [self.progressView runSpinnerAnimation];
            break;
        case RJTHudStyleProgress:
            [self.progressView runBasicAnimation];
            break;
        default:
            break;
    }
}

@end
