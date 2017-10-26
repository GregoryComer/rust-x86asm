use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtuqq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 137, 122, 197], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 139, 122, 11], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 254, 138, 122, 214], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM13)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 254, 140, 122, 40], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 175, 122, 249], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EDI, 1778217824, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 170, 122, 191, 96, 115, 253, 105], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 254, 174, 122, 201], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 140797324, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 169, 122, 52, 93, 140, 101, 100, 8], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 156, 122, 192], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1900493177, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 201, 122, 28, 189, 121, 57, 71, 113], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 254, 157, 122, 237], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1627787288, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 254, 205, 122, 188, 118, 24, 16, 6, 97], OperandSize::Qword)
}

