<?php

namespace App\DTOs;


class UserDTO {

    public function __construct
    (
        public int $id,
        public string $name,
        public string $email,
        public string $password,
        public ?string $bio,
        public ?string $avatarUrl,
        public int $links,
        public int $categories,
        public ?string $created_at,
        public ?string $updated_at,
        public ?string $deleted_at
    )
    {
    }


     public static function fromArray(array $data): self
     {
            return new self(
                $data['id'],
                $data['name'],
                $data['email'],
                $data['password'],
                $data['bio'],
                $data['avatarUrl'],
                $data['links'],
                $data['categories'],
                $data['created_at'],
                $data['updated_at'],
                $data['deleted_at']
            );
     }


    public function toArray(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'email' => $this->email,
            'password' => $this->password,
            'bio' => $this->bio,
            'avatarUrl' => $this->avatarUrl,
            'links' => $this->links,
            'categories' => $this->categories,
            'created_at' => $this->created_at,
            'updated_at' => $this->updated_at,
            'deleted_at' => $this->deleted_at
        ];
    }

}