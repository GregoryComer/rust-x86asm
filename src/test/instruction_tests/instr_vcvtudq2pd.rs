use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtudq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 122, 246], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1885439503, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 122, 36, 77, 15, 134, 97, 112], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 126, 141, 122, 223], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 2036203536, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 126, 137, 122, 188, 194, 16, 0, 94, 121], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 122, 242], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 173, 122, 4, 243], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 126, 170, 122, 213], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 172, 122, 60, 193], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 122, 192], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 122, 25], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 126, 204, 122, 224], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM14)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 126, 201, 122, 48], OperandSize::Qword)
}

