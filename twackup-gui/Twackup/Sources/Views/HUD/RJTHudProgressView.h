//
//  RJTHudProgressView.h
//  RJTranslate
//
//  Created by Даниил on 28/10/2018.
//  Copyright © 2018 Даниил. All rights reserved.
//

#import <UIKit/UIKit.h>

NS_ASSUME_NONNULL_BEGIN

#if defined(__IPHONE_14_0) || defined(__MAC_10_16) || defined(__TVOS_14_0) || defined(__WATCHOS_7_0)
#define RJTHUD_OBJC_DIRECT_MEMBERS __attribute__((objc_direct_members))
#define RJTHUD_OBJC_DIRECT __attribute__((objc_direct))
#define RJTHUD_DIRECT ,direct
#else
#define RJTHUD_OBJC_DIRECT_MEMBERS
#define RJTHUD_OBJC_DIRECT
#define RJTHUD_DIRECT
#endif

@interface RJTHudProgressView : UIView

+ (instancetype)defaultProgressView RJTHUD_OBJC_DIRECT;

@property (assign, nonatomic RJTHUD_DIRECT) CGFloat progress;

@property (assign, nonatomic, readonly RJTHUD_DIRECT) BOOL animating;

@property (strong, nonatomic, readonly RJTHUD_DIRECT) NSLayoutConstraint *heightConstraint;
@property (assign, nonatomic, readonly RJTHUD_DIRECT) CGFloat cachedHeight;

- (void)setProgress:(CGFloat)progress animated:(BOOL)animated RJTHUD_OBJC_DIRECT;

- (void)runBasicAnimation RJTHUD_OBJC_DIRECT;
- (void)runSpinnerAnimation RJTHUD_OBJC_DIRECT;
- (void)stopAnimating RJTHUD_OBJC_DIRECT;

@end

NS_ASSUME_NONNULL_END
