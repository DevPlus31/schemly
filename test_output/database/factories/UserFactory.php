<?php

namespace Database\Factories;

use Illuminate\Database\Eloquent\Factories\Factory;
use App\Models\User;

class UserFactory extends Factory
{
    protected $model = User::class;

    public function definition(): array
    {
        return [
            'name' => fake()->name(),
            'email' => fake()->email(),
            'password' => fake()->password(),
            'bio' => fake()->word(),
            'avatarUrl' => fake()->word(),
            'links' => fake()->numberBetween(1, 100),
            'categories' => fake()->numberBetween(1, 100),
        ];
    }
}
