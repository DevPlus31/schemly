<?php

namespace Database\Factories;

use Illuminate\Database\Eloquent\Factories\Factory;
use App\Models\Category;

class CategoryFactory extends Factory
{
    protected $model = Category::class;

    public function definition(): array
    {
        return [
            'userId' => fake()->numberBetween(1, 100),
            'name' => fake()->name(),
            'user' => fake()->numberBetween(1, 100),
            'links' => fake()->numberBetween(1, 100),
        ];
    }
}
