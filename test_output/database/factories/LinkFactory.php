<?php

namespace Database\Factories;

use Illuminate\Database\Eloquent\Factories\Factory;
use App\Models\Link;

class LinkFactory extends Factory
{
    protected $model = Link::class;

    public function definition(): array
    {
        return [
            'userId' => fake()->numberBetween(1, 100),
            'categoryId' => fake()->numberBetween(1, 100),
            'title' => fake()->sentence(3),
            'url' => fake()->url(),
            'isActive' => fake()->boolean(),
            'order' => fake()->numberBetween(1, 100),
            'user' => fake()->numberBetween(1, 100),
            'category' => fake()->numberBetween(1, 100),
        ];
    }
}
