//
//  RJTHud.h
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

/**
 Определяет стиль индикатора.

 - RJTHudStyleSpinner: Показывает спиннер и текст (если есть).
 - RJTHudStyleTextOnly: Показывает только текст.
 */
typedef NS_ENUM(NSInteger, RJTHudStyle) {
    RJTHudStyleProgress = 0,
    RJTHudStyleSpinner,
    RJTHudStyleTextOnly
};


@interface RJTHud : UIView

/**
 Выполняет показ индикатора на видимом окне UIWindow.

 @return Возвращает экземпляр индикатора для дальнейшей настройки.
 */
+ (nullable instancetype)show RJTHUD_OBJC_DIRECT;


/**
 @brief Устанавливает прогресс спиннера
 
 @discussion По умолчанию, установлен на 0.75.
 */
@property (assign, nonatomic RJTHUD_DIRECT) CGFloat progress;

/**
 Устанавливает стиль индикатора.
 */
@property (readwrite, assign, nonatomic RJTHUD_DIRECT) RJTHudStyle style;

/**
 Устанавливает основной текст в индикатор.
 Для скрытия должен быть установлен в nil.
 */
@property (readwrite, copy, nonatomic, nullable RJTHUD_DIRECT) NSString *text;

/**
 Устанавливает дополнительный текст в индикатор.
 Для скрытия должен быть установлен в nil.
 */
@property (readwrite, copy, nonatomic, nullable RJTHUD_DIRECT) NSString *detailedText;

/**
 Стиль заднего размытия. По умолчанию, UIBlurEffectStyleSystemThickMaterial
 */
@property (assign, nonatomic RJTHUD_DIRECT) UIBlurEffectStyle blurStyle;

/**
 Устанавливает прогресс спиннера.

 @param progress Значение прогресса от 0 до 1
 @param animated Если задан YES, то прогресс будет установлен с анимацией. В противном случае - без.
 */
- (void)setProgress:(CGFloat)progress animated:(BOOL)animated  RJTHUD_OBJC_DIRECT;

/**
 Выполняет показ индикатора на окне.

 @param animated Если флаг установлен в YES, выполняется с анимацией. В противном случае - без.
 @param view Представление, на котором необходимо показать индикатор.
 */
- (void)showAnimated:(BOOL)animated onView:(UIView *)view  RJTHUD_OBJC_DIRECT;

/**
 Выполняет скрытие индикатора.

 @param animated Если флаг установлен в YES, выполняется с анимацией. В противном случае - без.
 */
- (void)hideAnimated:(BOOL)animated  RJTHUD_OBJC_DIRECT;

/**
 Выполняет анимированное скрытие индикатора с задержкой.

 @param delay Время перед скрытием индикатора.
 */
- (void)hideAfterDelay:(CGFloat)delay  RJTHUD_OBJC_DIRECT;

- (void)performOnMainThread:(void(^)(void))block RJTHUD_OBJC_DIRECT;

@end

NS_ASSUME_NONNULL_END
