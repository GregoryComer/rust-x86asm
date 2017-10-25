use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvttps2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 120, 251], OperandSize::Dword)
}

fn vcvttps2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1656544053, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 120, 132, 151, 53, 219, 188, 98], OperandSize::Dword)
}

fn vcvttps2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 125, 138, 120, 196], OperandSize::Qword)
}

fn vcvttps2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDI, 380199498, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 120, 167, 74, 98, 169, 22], OperandSize::Qword)
}

fn vcvttps2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 120, 204], OperandSize::Dword)
}

fn vcvttps2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 674460287, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 120, 60, 197, 127, 114, 51, 40], OperandSize::Dword)
}

fn vcvttps2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 125, 173, 120, 243], OperandSize::Qword)
}

fn vcvttps2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 462946643, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 125, 172, 120, 164, 243, 83, 1, 152, 27], OperandSize::Qword)
}

fn vcvttps2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 155, 120, 200], OperandSize::Dword)
}

fn vcvttps2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1477744473, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 120, 156, 88, 89, 151, 20, 88], OperandSize::Dword)
}

fn vcvttps2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 125, 155, 120, 239], OperandSize::Qword)
}

fn vcvttps2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 461963991, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 125, 204, 120, 140, 145, 215, 2, 137, 27], OperandSize::Qword)
}

