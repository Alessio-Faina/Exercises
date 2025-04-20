#include <bits/stdc++.h>

template<typename T>
class shared_ptr {
	int* instances;
	T* m_ptr;

	public:
	shared_ptr() noexcept {
		m_ptr = nullptr;
		if (!instances) {
			instances = new int(0);
		}
	}

	shared_ptr(T* obj) noexcept
		: instances(nullptr)
 		, m_ptr(obj)
	{
		instances = new int(1);
		std::cout << "[SP]+ " << m_ptr << std::endl;
	};

	shared_ptr(const shared_ptr<T> &x) noexcept {
		m_ptr = x.m_ptr;
		instances = x.instances;
		(*instances)++;
	}

	shared_ptr operator=(const shared_ptr<T> &x) noexcept {
		m_ptr = x.m_ptr;
		instances = x.instances;
		(*instances)++;
	}

	T& operator* () noexcept {
		return *m_ptr;
	}
	T* operator-> () noexcept {
		return m_ptr;
	}
	
	~shared_ptr() noexcept {
		static_assert(sizeof(T) > 0);
		(*instances)--;
		if (*instances == 0) {
			delete m_ptr;
			std::cout << "[SP]- " << m_ptr << std::flush << std::endl;
			delete instances;
		}
	}

	T* get() noexcept {
		return m_ptr;
	}
};

template<typename T>
class unique_ptr {
	T* m_ptr{};

	public:
	unique_ptr() noexcept
	{
		m_ptr = nullptr;
	}

	unique_ptr(T* obj) noexcept
 		: m_ptr(obj)
	{
		std::cout << "[UP]+ " << m_ptr << std::endl;
	};

	unique_ptr(unique_ptr<T> const &) = delete;
	unique_ptr operator=(unique_ptr<T> const &) = delete;
	bool operator==(std::nullptr_t const &) noexcept
 	{
		return (m_ptr == nullptr);
	}
	bool operator!=(std::nullptr_t const &) noexcept
 	{
		return (m_ptr != nullptr);
	}


	// MOVE Semantics
	unique_ptr(unique_ptr&& x) noexcept
	{
		m_ptr = x.m_ptr;
		x.reset();
	};

	T& operator* () noexcept
 	{
		return *m_ptr;
	}
	T* operator-> () noexcept
 	{
		return m_ptr;
	}
	
	~unique_ptr() noexcept
 	{
		static_assert(sizeof(T) > 0);
		if (m_ptr) {
			std::cout << "[UP]- " << m_ptr << std::endl;
			delete m_ptr;
		}
	}

	T* get() noexcept
	{
		return m_ptr;
	}

	void reset() {
		m_ptr = nullptr;
	}
};

template<typename T, typename... Args>
unique_ptr<T> make_unique(Args&&... args) noexcept
{
	T* m_ptr = new T(std::forward<Args>(args)...);
	return m_ptr;
};

// MOVE IMPLEMENTATION =======================================

// Simplified std::remove_reference implementation
template <typename T>
struct remove_reference {
	using type = T;
};

// Specialization for lvalue references (e.g., int&)
template <typename T>
struct remove_reference<T&> {
	using type = T;
};

// Specialization for rvalue references (e.g., int&&)
template <typename T>
struct remove_reference<T&&> {
	using type = T;
};

// Simplified std::move implementation
template <typename T>
typename remove_reference<T>::type&& move(T&& arg) {
	// This function casts its argument into an rvalue reference,
	// signaling that the object can be moved from.
	return static_cast<typename remove_reference<T>::type&&>(arg);
}

// END: MOVE IMPLEMENTATION =======================================

//template<typename T>
//typename std::remove_reference<T>::type&& move(T&& arg) {
//	return ((T&&)arg);
//}

class test {
	int m_a;
	int m_b;
	public:
	test(int _a, int _b) {
		m_a = _a;
		m_b = _b;
	}

	int& a() { return m_a;}
	int sum() { return m_a + m_b; }
	int sub() { return m_a - m_b; }
};

void checkMoveRef(unique_ptr<test> &a) {
	std::cout << "MoveRef works! " << a->sum() << std::endl;
}

void checkMove(unique_ptr<test> a) {
	std::cout << "Move works! " << a->sum() << std::endl;
}

int main() {
	unique_ptr<test> A = make_unique<test>(1, 2);

	std::cout << A->sum() << std::endl;

	//unique_ptr<test> B = A;
	//
	test* T = new test(3, 4);

	//delete T;
	shared_ptr<test> B = T;
	std::cout << B->sum() << std::endl;


	shared_ptr<test> C = B;
	C->a() = 5;
	std::cout << "B:" << B->sum() << std::endl;
	std::cout << "C:" << C->sum() << std::endl;

	checkMoveRef(A);
	if (A != nullptr) {
		std::cout << "A is not null as expected!" << std::endl;
	} else {
		assert(false);
	}

	std::cout << A->sum() << std::endl;
	checkMove(move(A));
	if (A == nullptr) {
		std::cout << "A is null as expected!" << std::endl;
	} else { 
		assert(false);
	}
	return 0;
}

