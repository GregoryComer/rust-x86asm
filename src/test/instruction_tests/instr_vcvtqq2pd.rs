use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtqq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 142, 230, 244], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 612884957, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 140, 230, 188, 254, 221, 225, 135, 36], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 254, 142, 230, 249], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 786049130, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 254, 143, 230, 4, 189, 106, 40, 218, 46], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 171, 230, 243], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EAX, 1281305709, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 173, 230, 160, 109, 44, 95, 76], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 254, 171, 230, 195], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 254, 171, 230, 60, 158], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 185, 230, 219], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 201, 230, 24], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 254, 187, 230, 206], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 254, 205, 230, 60, 71], OperandSize::Qword)
}

