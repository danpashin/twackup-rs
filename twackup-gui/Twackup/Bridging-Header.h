//
//  Twackup-Bridging-Header.h
//  Twackup
//
//  Created by Daniil on 24.11.2022.
//

#ifndef Twackup_Bridging_Header_h
#define Twackup_Bridging_Header_h

#import "twackup.h"
#import "libroot.h"
#import "libSandy.h"
#import <UIKit/UIScrollView.h>

@interface UIScrollView (Private)
@property (assign, nonatomic, readonly, getter=_minimumContentOffset) CGPoint minimumContentOffset;
@property (assign, nonatomic, readonly, getter=_maximumContentOffset) CGPoint maximumContentOffset;
@end

#endif /* Twackup_Bridging_Header_h */
