#include <iostream>

#include "xtensor/xarray.hpp"
#include "xtensor/xio.hpp"
#include "xtensor/xview.hpp"

#include "xtensor/xbuilder.hpp"
#include "xtensor/xmath.hpp"
#include "xtensor-io/ximage.hpp"

int main()
{
    // loads png image into xarray with shape HEIGHT x WIDTH x CHANNELS
    auto arr = xt::load_image("test.png");
}