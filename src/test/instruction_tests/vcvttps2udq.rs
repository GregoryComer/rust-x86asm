use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvttps2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 120, 217], OperandSize::Dword)
}

fn vcvttps2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 120, 12, 147], OperandSize::Dword)
}

fn vcvttps2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 124, 143, 120, 248], OperandSize::Qword)
}

fn vcvttps2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(XMM11)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 124, 140, 120, 28, 153], OperandSize::Qword)
}

fn vcvttps2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 120, 192], OperandSize::Dword)
}

fn vcvttps2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(ECX, 209340880, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 171, 120, 185, 208, 73, 122, 12], OperandSize::Dword)
}

fn vcvttps2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 124, 173, 120, 201], OperandSize::Qword)
}

fn vcvttps2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1379733258, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 124, 170, 120, 148, 192, 10, 15, 61, 82], OperandSize::Qword)
}

fn vcvttps2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 157, 120, 233], OperandSize::Dword)
}

fn vcvttps2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 203, 120, 47], OperandSize::Dword)
}

fn vcvttps2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 124, 158, 120, 228], OperandSize::Qword)
}

fn vcvttps2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UDQ, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1906445736, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 124, 204, 120, 148, 74, 168, 13, 162, 113], OperandSize::Qword)
}

